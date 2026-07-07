"""Exact placement finding for letter-composed targets (tests 01-03).

For clipart L with opaque mask m at offset (r,c):
SSD(r,c) = sum m*(T-L)^2 over the overlap = sum(m*L^2) - 2*corr(T, m*L) + corr(T^2, m).
Positions with SSD == 0 (over full mask, in-bounds) are exact matches.
"""
import numpy as np
from scipy.signal import fftconvolve


def ssd_map(target, clip_rgb, clip_alpha):
    """target: HxWx3 float64. Returns (H-ch+1)x(W-cw+1) SSD over opaque mask
    for all fully-in-bounds placements."""
    h, w = target.shape[:2]
    ch, cw = clip_alpha.shape
    m = clip_alpha.astype(np.float64)
    total = np.zeros((h - ch + 1, w - cw + 1))
    for k in range(3):
        t = target[:, :, k]
        l = clip_rgb[:, :, k].astype(np.float64) * m
        c1 = float((l * l).sum())
        corr_tl = fftconvolve(t, l[::-1, ::-1], mode="valid")
        corr_t2m = fftconvolve(t * t, m[::-1, ::-1], mode="valid")
        total += c1 - 2 * corr_tl + corr_t2m
    return total


def find_exact(target, clips, tol_per_px=0.5, min_frac=1.0):
    """Return list of (pic, r, c, ssd) for near-zero SSD placements."""
    tf = target.astype(np.float64)
    hits = []
    for i, clip in enumerate(clips):
        if clip["alpha"].shape[0] > target.shape[0] or clip["alpha"].shape[1] > target.shape[1]:
            continue
        area = int(clip["alpha"].sum())
        s = ssd_map(tf, clip["rgb"], clip["alpha"])
        thr = tol_per_px * area
        ys, xs = np.where(s <= thr)
        for y, x in zip(ys, xs):
            hits.append((i + 1, int(y), int(x), float(s[y, x])))
    return hits
