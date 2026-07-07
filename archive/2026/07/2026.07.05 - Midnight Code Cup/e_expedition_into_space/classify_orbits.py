#!/usr/bin/env python3
"""Classify star orbits by direction / angular speed / radius and visualize."""
import sys
import numpy as np
from PIL import Image

sys.path.insert(0, '..')
from analyze_stars import fit_circle  # noqa

import glob
import re
from scipy import ndimage


def build_tracks(lo, hi):
    files = [f't{i}.png' for i in range(lo, hi + 1)]
    frames = [np.array(Image.open(f), dtype=np.int16) for f in files]
    stack = np.stack(frames)
    std = stack.std(axis=0)
    static_mask = std < 8
    static_mask[440:590, 370:660] = True

    tracks = []
    active = []
    for fi, img in enumerate(frames):
        work = img.copy()
        work[static_mask] = 0
        lab, n = ndimage.label(work > 40)
        if n:
            cents = ndimage.center_of_mass(work, lab, range(1, n + 1))
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
    return tracks


def main():
    lo, hi = int(sys.argv[1]), int(sys.argv[2])
    tracks = build_tracks(lo, hi)
    rows = []
    for t in tracks:
        if len(t) < 40:
            continue
        f = np.array([p[0] for p in t], dtype=float)
        xs = np.array([p[1] for p in t])
        ys = np.array([p[2] for p in t])
        if np.hypot(xs.max() - xs.min(), ys.max() - ys.min()) < 4:
            continue
        cx, cy, r, rms = fit_circle(xs, ys)
        if rms > 1.0 or r < 2 or r > 500:
            continue
        ang = np.unwrap(np.arctan2(ys - cy, xs - cx))
        # angular velocity per frame via linear fit
        w = np.polyfit(f, ang, 1)[0]
        rows.append((cx, cy, r, w, xs[0], ys[0]))
        print(f'center=({cx:7.1f},{cy:7.1f}) r={r:6.1f} w={w:+.4f} rad/frame period={2*np.pi/abs(w):6.1f}f')
    rows = np.array(rows)
    np.save('orbits.npy', rows)
    print(len(rows), 'orbits')
    print('CW (w>0):', (rows[:, 3] > 0).sum(), 'CCW:', (rows[:, 3] < 0).sum())
    # RGB visualization: red = CW, green = CCW, at centers; brightness by |w|
    img = np.zeros((1000, 1000, 3), dtype=np.uint8)
    for cx, cy, r, w, x0, y0 in rows:
        ix, iy = int(round(cx)), int(round(cy))
        if 0 <= ix < 1000 and 0 <= iy < 1000:
            ch = 0 if w > 0 else 1
            img[max(0, iy - 3):iy + 4, max(0, ix - 3):ix + 4, ch] = 255
    Image.fromarray(img).save('orbit_dirs.png')
    print('saved orbit_dirs.png')


if __name__ == '__main__':
    main()
