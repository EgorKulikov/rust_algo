"""Noise-averaged signed tile map for a Hamming-style beast (B).

B's score ~ 100*(1 - hamming(image, target)/65536), so score(all-black with one
tile flipped white) - score(all-black) is positive iff that tile is target-white.
B is noisy (~0.2), so we average k reads per tile and a big baseline. Result maps
the target: positive delta = target-white region. Writes tilemap_avg_<name>_<t>.json.
"""
import sys
import json
import os
import numpy as np
import mcc

N = mcc.N


def run(name, tile=16, k=4, base_k=16):
    beast = mcc.Beast(name)
    out = os.path.join(mcc.HERE, f"tilemap_avg_{name}_{tile}.json")
    data = json.load(open(out)) if os.path.exists(out) else {}
    black = np.zeros((N, N), bool)
    baseline = float(np.mean([beast.query(black, tag="avgbase")[0] for _ in range(base_k)]))
    data["_baseline"] = baseline
    print(f"[{name}] baseline={baseline:.3f}", flush=True)
    g = N // tile
    order = sorted(range(g), key=lambda r: abs(r - g // 2))  # center rows first
    for i in order:
        for j in range(g):
            key = f"{i},{j}"
            if key in data:
                continue
            if beast.key:
                break
            im = black.copy()
            im[i * tile:(i + 1) * tile, j * tile:(j + 1) * tile] = True
            sc = float(np.mean([beast.query(im, tag=f"avg{tile}:{key}")[0] for _ in range(k)]))
            data[key] = sc - baseline
            if (i * g + j) % 8 == 0:
                json.dump(data, open(out, "w"))
    json.dump(data, open(out, "w"))
    print(f"[{name}] tilemap_avg {tile} done", flush=True)


if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "b"
    tile = int(sys.argv[2]) if len(sys.argv) > 2 else 16
    k = int(sys.argv[3]) if len(sys.argv) > 3 else 4
    run(name, tile, k=k)
