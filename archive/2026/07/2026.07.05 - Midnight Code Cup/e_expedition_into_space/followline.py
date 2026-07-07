#!/usr/bin/env python3
"""Follow a road line to the next crossroads.

Usage:
  followline.py test <img.png>          # detect lines in an image, print them
  followline.py go <dirx> <diry>        # follow line from current pos in given
                                        # initial direction until a crossroads
"""
import math
import sys
import time

import numpy as np
from PIL import Image

sys.path.insert(0, '..')
sys.path.insert(0, '.')
from nav import goto, state, view

THRESH = 60
CLOCK = (slice(470, 532), slice(455, 550))  # ship sprite only


def bright_points(img):
    """Pixels belonging to long components only (roads, not stars/text)."""
    from scipy import ndimage
    m = img.copy()
    m[CLOCK] = 0
    mask = m > THRESH
    lab, n = ndimage.label(mask, structure=np.ones((3, 3)))
    if n == 0:
        return np.array([]), np.array([])
    objs = ndimage.find_objects(lab)
    keep = np.zeros(n + 1, bool)
    for i, sl in enumerate(objs, 1):
        h = sl[0].stop - sl[0].start
        w = sl[1].stop - sl[1].start
        if max(h, w) >= 150 or h + w >= 220:
            keep[i] = True
    ys, xs = np.nonzero(keep[lab])
    return xs.astype(float), ys.astype(float)


def detect_lines(img, min_inliers=170, max_gap=120, min_len=170, min_fill=0.9):
    """Hough on bright pixels. Returns list of dicts with theta (normal angle),
    rho, endpoints of the densely-supported segment."""
    xs, ys = bright_points(img)
    if len(xs) == 0:
        return []
    thetas = np.deg2rad(np.arange(0, 180, 0.5))
    ct, st = np.cos(thetas), np.sin(thetas)
    # rho for each point x theta:  (N, T)
    rho = np.outer(xs, ct) + np.outer(ys, st)
    RBIN = 3.0
    rbin = np.round(rho / RBIN).astype(np.int64)
    cands = []
    off = rbin.min()
    nb = rbin.max() - off + 1
    T = len(thetas)
    counts = np.zeros((T, nb), dtype=np.int32)
    for ti in range(T):
        np.add.at(counts[ti], rbin[:, ti] - off, 1)
    # candidate bins
    ti_arr, bi_arr = np.nonzero(counts >= min_inliers // 2)
    seen = []
    for ti, bi in zip(ti_arr, bi_arr):
        th = thetas[ti]
        r0 = (bi + off) * RBIN
        d = np.abs(xs * math.cos(th) + ys * math.sin(th) - r0)
        sel = d < 3.5
        if sel.sum() < min_inliers:
            continue
        # project inliers along the line
        ux, uy = -math.sin(th), math.cos(th)
        t = xs[sel] * ux + ys[sel] * uy
        t.sort()
        # densest run with gaps < max_gap
        best_i0, best_i1 = 0, 0
        i0 = 0
        for i in range(1, len(t)):
            if t[i] - t[i - 1] > max_gap:
                if t[i - 1] - t[i0] > t[best_i1] - t[best_i0]:
                    best_i0, best_i1 = i0, i - 1
                i0 = i
        if t[-1] - t[i0] > t[best_i1] - t[best_i0]:
            best_i0, best_i1 = i0, len(t) - 1
        seg_len = t[best_i1] - t[best_i0]
        n_seg = best_i1 - best_i0 + 1
        if seg_len < min_len or n_seg < min_inliers:
            continue
        # fill ratio: line px are contiguous; stars/text give sparse support
        fill = n_seg / max(seg_len, 1)
        if fill < min_fill:
            continue
        # refine rho/theta with PCA of the segment inliers
        selt = np.argsort(xs[sel] * ux + ys[sel] * uy)[best_i0:best_i1 + 1]
        sx, sy = xs[sel][selt], ys[sel][selt]
        mx, my = sx.mean(), sy.mean()
        cov = np.cov(np.stack([sx - mx, sy - my]))
        evals, evecs = np.linalg.eigh(cov)
        v = evecs[:, np.argmax(evals)]  # direction along line
        th_ref = math.atan2(v[0], -v[1])  # normal angle
        if th_ref < 0:
            th_ref += math.pi
        r_ref = mx * math.cos(th_ref) + my * math.sin(th_ref)
        p0 = (sx[0], sy[0])
        p1 = (sx[-1], sy[-1])
        cand = dict(theta=th_ref, rho=r_ref, n=n_seg, len=seg_len, fill=fill,
                    p0=p0, p1=p1, mid=((p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2))
        # NMS
        dup = False
        for c in seen:
            dth = abs(c['theta'] - cand['theta'])
            dth = min(dth, math.pi - dth)
            if dth < math.radians(4) and abs(c['rho'] - cand['rho']) < 25:
                if cand['n'] > c['n']:
                    c.update(cand)
                dup = True
                break
        if not dup:
            seen.append(cand)
    seen.sort(key=lambda c: -c['n'])
    return seen


def line_dir(line):
    """Unit vector along line."""
    th = line['theta']
    return (-math.sin(th), math.cos(th))


def dist_to_line(px, py, line):
    return abs(px * math.cos(line['theta']) + py * math.sin(line['theta']) - line['rho'])


def foot(px, py, line):
    d = px * math.cos(line['theta']) + py * math.sin(line['theta']) - line['rho']
    return px - d * math.cos(line['theta']), py - d * math.sin(line['theta'])


def intersect(l1, l2):
    a1, b1, c1 = math.cos(l1['theta']), math.sin(l1['theta']), l1['rho']
    a2, b2, c2 = math.cos(l2['theta']), math.sin(l2['theta']), l2['rho']
    det = a1 * b2 - a2 * b1
    if abs(det) < 1e-9:
        return None
    return ((c1 * b2 - c2 * b1) / det, (a1 * c2 - a2 * c1) / det)


def count_arms(img, ix, iy, rmin=40, rmax=250):
    """Count directions in which road pixels radiate from (ix, iy)."""
    xs, ys = bright_points(img)
    dx, dy = xs - ix, ys - iy
    d = np.hypot(dx, dy)
    sel = (d > rmin) & (d < rmax)
    if sel.sum() < 10:
        return 0
    ang = np.arctan2(dy[sel], dx[sel])
    bins = np.zeros(72)
    np.add.at(bins, (np.floor((ang + np.pi) / (2 * np.pi) * 72).astype(int)) % 72, 1)
    k = np.array([1, 2, 3, 2, 1], dtype=float)
    sm = np.convolve(np.concatenate([bins[-2:], bins, bins[:2]]), k, 'same')[2:-2]
    thr = max(6.0, sm.max() * 0.25)
    peaks = []
    for i in range(72):
        if sm[i] >= thr and sm[i] >= sm[(i - 1) % 72] and sm[i] > sm[(i + 1) % 72]:
            peaks.append(i)
    # merge peaks closer than 25 deg (5 bins) circularly
    merged = []
    for p in peaks:
        if any(min((p - q) % 72, (q - p) % 72) < 5 for q in merged):
            continue
        merged.append(p)
    return len(merged)


def snap(name):
    info = view(name)
    img = np.array(Image.open(name), dtype=np.int16)
    return img, info['position-x'], info['position-y']


def follow(dx, dy, hop=650, max_hops=60):
    dirw = np.array([dx, dy], dtype=float)
    dirw /= np.linalg.norm(dirw)
    for step in range(max_hops):
        img, wx, wy = snap(f'fl_{step}.png')
        lines = detect_lines(img)
        if not lines:
            print(f'step {step}: NO LINES at ({wx:.0f},{wy:.0f}) — stopping', flush=True)
            return
        # our line: closest to ship among lines roughly parallel to dirw
        def angcos(l):
            u = line_dir(l)
            return abs(u[0] * dirw[0] + u[1] * dirw[1])
        ours = [l for l in lines if angcos(l) > 0.85]
        if not ours:
            ours = [l for l in lines if angcos(l) > 0.5]  # sharp curve fallback
        if not ours:
            print(f'step {step}: no parallel line (have {len(lines)}) — stopping', flush=True)
            return
        cur = min(ours, key=lambda l: dist_to_line(500, 500, l))
        d0 = dist_to_line(500, 500, cur)
        # crossroads check: any other line crossing ours in view
        for other in lines:
            if other is cur:
                continue
            u1, u2 = line_dir(cur), line_dir(other)
            cross = abs(u1[0] * u2[1] - u1[1] * u2[0])
            if cross < math.sin(math.radians(15)):
                continue
            pt = intersect(cur, other)
            if pt is None:
                continue
            ix, iy = pt
            if 20 <= ix <= 980 and 20 <= iy <= 980:
                ahead = (ix - 500) * dirw[0] + (iy - 500) * dirw[1]
                if ahead < -100:
                    continue  # behind us — the junction we just left
                arms = count_arms(img, ix, iy)
                wix, wiy = wx + ix - 500, wy + iy - 500
                if arms >= 3:
                    print(f'step {step}: CROSSROADS ({arms} arms) at world ({wix:.0f},{wiy:.0f}) '
                          f'angles {math.degrees(cur["theta"]):.1f}/{math.degrees(other["theta"]):.1f}',
                          flush=True)
                    goto(wix, wiy, tol=25)
                    view('crossroads.png')
                    return
                print(f'step {step}: curve ({arms} arms) at ({wix:.0f},{wiy:.0f}) — continuing',
                      flush=True)
        # next waypoint: foot of ship on line + hop along dir
        u = np.array(line_dir(cur))
        if u @ dirw < 0:
            u = -u
        fx, fy = foot(500, 500, cur)
        nx, ny = fx + u[0] * hop, fy + u[1] * hop
        wnx, wny = wx + nx - 500, wy + ny - 500
        print(f'step {step}: at ({wx:.0f},{wy:.0f}) line d={d0:.0f}px '
              f'th={math.degrees(cur["theta"]):.1f} n={cur["n"]} -> goto ({wnx:.0f},{wny:.0f})',
              flush=True)
        dirw = u
        goto(wnx, wny, tol=40)
    print('max hops reached', flush=True)


if __name__ == '__main__':
    if sys.argv[1] == 'test':
        img = np.array(Image.open(sys.argv[2]), dtype=np.int16)
        for l in detect_lines(img):
            print(f"th={math.degrees(l['theta']):6.1f} rho={l['rho']:7.1f} n={l['n']:4d} "
                  f"len={l['len']:5.0f} fill={l['fill']:.2f} p0={l['p0']} p1={l['p1']}")
    elif sys.argv[1] == 'go':
        follow(float(sys.argv[2]), float(sys.argv[3]))
