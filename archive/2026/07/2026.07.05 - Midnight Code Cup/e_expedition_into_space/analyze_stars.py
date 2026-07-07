#!/usr/bin/env python3
"""Track moving stars across captured frames, fit a circle to each track,
and plot the fitted orbit centers — looking for a hidden pattern."""
import glob
import re
import sys

import numpy as np
from PIL import Image
from scipy import ndimage
from scipy.spatial import cKDTree


def detect_blobs(img, static_mask, thresh=40):
    work = img.copy()
    work[static_mask] = 0
    lab, n = ndimage.label(work > thresh)
    if n == 0:
        return np.zeros((0, 2))
    cents = ndimage.center_of_mass(work, lab, range(1, n + 1))
    return np.array([(c[1], c[0]) for c in cents])  # x, y


def fit_circle(xs, ys):
    A = np.column_stack([xs, ys, np.ones(len(xs))])
    b = xs**2 + ys**2
    sol, *_ = np.linalg.lstsq(A, b, rcond=None)
    cx, cy = sol[0] / 2, sol[1] / 2
    r = np.sqrt(max(sol[2] + cx**2 + cy**2, 0))
    resid = np.sqrt((xs - cx) ** 2 + (ys - cy) ** 2) - r
    return cx, cy, r, np.sqrt(np.mean(resid**2))


def main():
    lo, hi = int(sys.argv[1]), int(sys.argv[2])
    files = [f't{i}.png' for i in range(lo, hi + 1)]
    frames = [np.array(Image.open(f), dtype=np.int16) for f in files]
    print(f'{len(frames)} frames')
    stack = np.stack(frames)
    std = stack.std(axis=0)
    static_mask = std < 8
    clock = np.zeros_like(static_mask)
    clock[440:590, 370:660] = True
    static_mask |= clock

    tracks = []   # list of lists of (frame, x, y)
    active = []   # track indices considered alive
    for fi, img in enumerate(frames):
        blobs = detect_blobs(img, static_mask)
        used = set()
        new_active = []
        # sort active tracks: ones with more history matched first
        for ti in sorted(active, key=lambda t: -len(tracks[t])):
            tr = tracks[ti]
            lf, lx, ly = tr[-1]
            if fi - lf > 2:
                continue  # dead
            # predict via last displacement
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
            for bi, (bx, by) in enumerate(blobs):
                if bi in used:
                    continue
                d = np.hypot(bx - ex, by - ey)
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

    lens = sorted((len(t) for t in tracks), reverse=True)
    print(f'{len(tracks)} tracks; top lengths {lens[:10]}; >=40: {sum(1 for l in lens if l >= 40)}')

    centers = []
    for t in tracks:
        if len(t) < 40:
            continue
        xs = np.array([p[1] for p in t])
        ys = np.array([p[2] for p in t])
        span = np.hypot(xs.max() - xs.min(), ys.max() - ys.min())
        if span < 4:
            continue
        cx, cy, r, rms = fit_circle(xs, ys)
        if rms < 1.0 and 2 < r < 500:
            centers.append((cx, cy, r, rms, len(t)))

    print(f'{len(centers)} good circle fits')
    out = np.zeros((1000, 1000), dtype=np.uint8)
    for cx, cy, r, rms, n in centers:
        ix, iy = int(round(cx)), int(round(cy))
        if 0 <= ix < 1000 and 0 <= iy < 1000:
            out[max(0, iy - 2):iy + 3, max(0, ix - 2):ix + 3] = 255
    Image.fromarray(out).save('centers.png')
    np.save('centers.npy', np.array([c[:3] for c in centers]))
    print('saved centers.png / centers.npy')


if __name__ == '__main__':
    main()
