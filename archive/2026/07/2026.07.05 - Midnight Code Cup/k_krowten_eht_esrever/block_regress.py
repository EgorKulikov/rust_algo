"""Compressed-sensing reconstruction of B's target on the 4x4-block grid.

B is linear: score(image) = base + (100/65536) * sum_p x_p * s_p, where s_p=+1 if
target-white else -1 (verified: random white% w -> score ~ 88 - 0.76w). If the
target lives on a 64x64 grid of 4x4 blocks (Egor's insight), there are only 4096
unknown block signs. Query many random block-subsets (white on black), then
regress score on the block-indicator design to recover each block's sign, and
paint white exactly the positive blocks -> should match the target closely.

Collects (mask_bits, score) into blockdata_<name>.npz incrementally (resumable).
The solver is block_solve.py.
"""
import sys
import os
import numpy as np
import mcc

N = mcc.N
C = 4                      # block size in pixels
G = N // C                 # 64 blocks per side
NB = G * G                 # 4096 blocks


def mask_to_img(bits):
    grid = bits.reshape(G, G)
    return np.kron(grid, np.ones((C, C), bool))


def collect(name, M=8000, p=0.5, seed0=1000):
    beast = mcc.Beast(name)
    path = os.path.join(mcc.HERE, f"blockdata_{name}.npz")
    masks, scores = [], []
    if os.path.exists(path):
        d = np.load(path)
        masks = list(d["masks"]); scores = list(d["scores"])
        print(f"[{name}] resume {len(scores)} samples", flush=True)
    start = len(scores)
    for i in range(start, M):
        if beast.key:
            break
        rng = np.random.default_rng(seed0 + i)
        bits = rng.random(NB) < p
        img = mask_to_img(bits)
        s, _ = beast.query(img, tag=f"reg{i}")
        masks.append(np.packbits(bits))
        scores.append(s)
        if (i + 1) % 50 == 0:
            np.savez(path, masks=np.array(masks), scores=np.array(scores))
            print(f"[{name}] {i+1}/{M} last={s:.2f}", flush=True)
    np.savez(path, masks=np.array(masks), scores=np.array(scores))
    print(f"[{name}] collect done {len(scores)}", flush=True)


if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "b"
    M = int(sys.argv[2]) if len(sys.argv) > 2 else 8000
    collect(name, M=M)
