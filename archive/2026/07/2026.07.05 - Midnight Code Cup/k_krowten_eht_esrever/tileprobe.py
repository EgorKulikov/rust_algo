"""Map a beast's target by tiling: black everywhere except one white tile.

For each tile in a GxG grid, send an image that is all-black except that single
tile painted white, and record the score. This reveals, per tile, whether white
is "allowed" there (score > 0, i.e. the target is white or don't-care) or "gated"
(score == 0, the target is black there), plus the contribution magnitude.

Writes tilemap_<name>_<tile>.json = {"i,j": score}. Resumable (skips logged).
"""
import sys
import json
import os
import numpy as np
import mcc

N = mcc.N


def run(name, tile=16):
    beast = mcc.Beast(name)
    out_path = os.path.join(mcc.HERE, f"tilemap_{name}_{tile}.json")
    data = {}
    if os.path.exists(out_path):
        data = json.load(open(out_path))
    g = N // tile
    for i in range(g):
        for j in range(g):
            k = f"{i},{j}"
            if k in data:
                continue
            if beast.key:
                break
            im = np.zeros((N, N), bool)
            im[i * tile:(i + 1) * tile, j * tile:(j + 1) * tile] = True
            s, _ = beast.query(im, tag=f"tile{tile}:{k}")
            data[k] = s
            if len(data) % 8 == 0:
                json.dump(data, open(out_path, "w"))
    json.dump(data, open(out_path, "w"))
    nz = {k: v for k, v in data.items() if v > 0}
    print(f"[{name}] tilemap {tile}px: {len(nz)}/{g*g} nonzero, "
          f"max={max(data.values()):.4f}")
    return data


if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "a"
    tile = int(sys.argv[2]) if len(sys.argv) > 2 else 16
    run(name, tile)
