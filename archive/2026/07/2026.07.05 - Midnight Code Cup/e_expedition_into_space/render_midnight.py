#!/usr/bin/env python3
"""Full pipeline: frames (+ meta.json with server millis) -> star orbit model
-> render at displayed-midnight T*.

Usage: render_midnight.py <prefix> <n_frames> [T*_utc]
"""
import json
import sys
import datetime
import numpy as np
from PIL import Image
from scipy import ndimage


def fit_circle(xs, ys):
    A = np.column_stack([xs, ys, np.ones(len(xs))])
    b = xs**2 + ys**2
    sol, *_ = np.linalg.lstsq(A, b, rcond=None)
    cx, cy = sol[0] / 2, sol[1] / 2
    r = np.sqrt(max(sol[2] + cx**2 + cy**2, 0))
    resid = np.sqrt((xs - cx) ** 2 + (ys - cy) ** 2) - r
    return cx, cy, r, np.sqrt(np.mean(resid**2))


def main():
    prefix = sys.argv[1]
    n = int(sys.argv[2])
    tstar_utc = sys.argv[3] if len(sys.argv) > 3 else '2026-07-04 10:00:00'

    meta = json.load(open(f'{prefix}meta.json'))
    assert len(meta) == n, (len(meta), n)
    ts = np.array([m['updated-at-millis'] for m in meta]) / 1000.0
    t0 = ts[0]
    print(f'span {ts[-1]-t0:.1f}s')

    frames = [np.array(Image.open(f'{prefix}{i}.png'), dtype=np.int16) for i in range(n)]
    stack = np.stack(frames)
    std = stack.std(axis=0)
    static_mask = std < 8
    static_mask[440:590, 370:660] = True  # clock + ship

    tracks = []
    active = []
    for fi, img in enumerate(frames):
        work = img.copy()
        work[static_mask] = 0
        lab, nb = ndimage.label(work > 40)
        if nb:
            cents = ndimage.center_of_mass(work, lab, range(1, nb + 1))
            blobs = np.array([(c[1], c[0]) for c in cents])
        else:
            blobs = np.zeros((0, 2))
        used = set()
        new_active = []
        for ti in sorted(active, key=lambda t: -len(tracks[t])):
            tr = tracks[ti]
            lf, lx, ly = tr[-1]
            if fi - lf > 2:
                continue
            if len(tr) >= 2:
                pf, px, py = tr[-2]
                dt = lf - pf
                vx, vy = (lx - px) / dt, (ly - py) / dt
                ex, ey = lx + vx * (fi - lf), ly + vy * (fi - lf)
                rad = 3.0
            else:
                ex, ey = lx, ly
                rad = 5.0
            best, bestd = None, rad
            for bi in range(len(blobs)):
                if bi in used:
                    continue
                d = np.hypot(blobs[bi][0] - ex, blobs[bi][1] - ey)
                if d < bestd:
                    best, bestd = bi, d
            if best is not None:
                used.add(best)
                tr.append((fi, blobs[best][0], blobs[best][1]))
            new_active.append(ti)
        for bi in range(len(blobs)):
            if bi not in used:
                tracks.append([(fi, blobs[bi][0], blobs[bi][1])])
                new_active.append(len(tracks) - 1)
        active = [ti for ti in new_active if fi - tracks[ti][-1][0] <= 2]

    stars = []
    for t in tracks:
        if len(t) < 20:
            continue
        f = np.array([p[0] for p in t])
        xs = np.array([p[1] for p in t])
        ys = np.array([p[2] for p in t])
        if np.hypot(xs.max() - xs.min(), ys.max() - ys.min()) < 3:
            continue
        cx, cy, r, rms = fit_circle(xs, ys)
        if rms > 1.5 or r < 2 or r > 500:
            continue
        tt = ts[f] - t0
        ang = np.unwrap(np.arctan2(ys - cy, xs - cx))
        w, _ = np.polyfit(tt, ang, 1)
        k = 2 * np.pi / (5 * abs(w))
        kr = round(k)
        if kr < 5 or kr > 40 or abs(k - kr) > 0.35:
            continue
        wsnap = np.sign(w) * 2 * np.pi / (5 * kr)
        phi = np.angle(np.exp(1j * (ang - wsnap * tt)).mean())
        resid = np.std(np.angle(np.exp(1j * (ang - wsnap * tt - phi))))
        if resid > 0.15:
            continue
        stars.append((cx, cy, r, wsnap, phi, kr, len(t)))
    stars = np.array(stars)
    print(len(stars), 'stars before dedupe')
    order = np.argsort(-stars[:, 6])
    keep = []
    for i in order:
        dup = False
        for j in keep:
            if (abs(stars[i, 5] - stars[j, 5]) < 0.5
                    and np.hypot(stars[i, 0] - stars[j, 0], stars[i, 1] - stars[j, 1]) < 4
                    and abs(stars[i, 2] - stars[j, 2]) < 4):
                dup = True
                break
        if not dup:
            keep.append(i)
    stars = stars[keep]
    print(len(stars), 'after dedupe; k values:', sorted(set(stars[:, 5].astype(int))))
    np.save(f'{prefix}stars.npy', stars)
    np.save(f'{prefix}t0.npy', np.array([t0]))

    S = (datetime.datetime.strptime(tstar_utc, '%Y-%m-%d %H:%M:%S')
         .replace(tzinfo=datetime.timezone.utc).timestamp())
    trel = S - t0
    ang = stars[:, 3] * trel + stars[:, 4]
    px = stars[:, 0] + stars[:, 2] * np.cos(ang)
    py = stars[:, 1] + stars[:, 2] * np.sin(ang)
    img = np.zeros((1000, 1000), np.uint8)
    for x, y in zip(px, py):
        ix, iy = int(round(x)), int(round(y))
        if 0 <= ix < 1000 and 0 <= iy < 1000:
            img[max(0, iy - 1):iy + 2, max(0, ix - 1):ix + 2] = 255
    out = f'{prefix}midnight.png'
    Image.fromarray(img).save(out)
    print('rendered', out, ' in-frame:', ((px >= 0) & (px < 1000) & (py >= 0) & (py < 1000)).sum())


if __name__ == '__main__':
    main()
