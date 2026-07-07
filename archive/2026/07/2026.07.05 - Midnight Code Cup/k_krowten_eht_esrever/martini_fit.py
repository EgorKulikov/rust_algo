"""Fit a parametric martini glass (bowl triangle + stem + base) for B's cup.

Circle fixed at R=118, t=4 (already nailed). Coordinate-search the glass params
against B (averaged). Bowl = inverted triangle apex-down; stem = thin vertical;
base = horizontal foot.
"""
import numpy as np
import mcc

N = mcc.N
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
cx = N // 2
r = np.sqrt((xx - cx) ** 2 + (yy - N / 2) ** 2)
CIRCLE = np.abs(r - 118) <= 2.0


def glass(top_y, half_w, apex_y, base_y, base_hw, stem_hw=3, base_t=4):
    img = np.zeros((N, N), bool)
    for y in range(top_y, apex_y):
        f = (y - top_y) / max(1, (apex_y - top_y))
        hw = int(round(half_w * (1 - f)))
        img[y, cx - hw:cx + hw + 1] = True
    img[apex_y:base_y, cx - stem_hw:cx + stem_hw + 1] = True
    img[base_y:base_y + base_t, cx - base_hw:cx + base_hw + 1] = True
    return img


def avg(cup, k=6, tag=""):
    return float(np.mean([b.query(CIRCLE | cup, tag=tag)[0] for _ in range(k)]))


# defaults from the cropped view
P = dict(top_y=85, half_w=35, apex_y=128, base_y=152, base_hw=30, stem_hw=3, base_t=5)
print("start", round(avg(glass(**P), 6, "m_start"), 3), flush=True)

grids = [
    ("top_y", [80, 84, 88, 92]),
    ("half_w", [28, 32, 36, 40, 44]),
    ("apex_y", [120, 125, 130, 135]),
    ("base_y", [146, 150, 154, 158]),
    ("base_hw", [22, 28, 34, 40]),
    ("stem_hw", [2, 3, 5]),
    ("base_t", [3, 5, 7]),
]
for name, vals in grids:
    best = (-1, P[name])
    for v in vals:
        Q = dict(P); Q[name] = v
        sc = avg(glass(**Q), 6, f"m_{name}{v}")
        if sc > best[0]:
            best = (sc, v)
        if b.key:
            print("KEY", b.key); raise SystemExit
    P[name] = best[1]
    print(f"{name} -> {best[1]} ({best[0]:.3f})", flush=True)
print("FINAL", P, round(avg(glass(**P), 8, "m_final"), 3), flush=True)
