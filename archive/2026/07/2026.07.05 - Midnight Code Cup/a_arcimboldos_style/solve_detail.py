"""Detail-dot phase for any test: load existing out/NN.out, then greedily
add small cliparts (bbox <= maxdim) on worst-error blocks until budget.

Usage: solve_detail.py DD [maxdim] [topk]
"""
import sys
import time

import numpy as np

sys.path.insert(0, ".")
import mcc_a


def main(dd, maxdim=260, topk=40, jitter=8, path=None):
    info = mcc_a.load_input(dd)
    n_budget = info["n"]
    target = mcc_a.load_target(info["target"]).astype(np.float32)
    clips = mcc_a.load_collection(info["collection"])
    h, w = target.shape[:2]
    path = path or f"out/{dd:02d}.out"
    placements = mcc_a.read_out(path)
    canvas = mcc_a.render(placements, (h, w), clips).astype(np.int16)
    print(f"loaded {len(placements)} placements, err={mcc_a.score(target.astype(np.int16), canvas):,}")

    ids = [i for i, c in enumerate(clips) if max(c["alpha"].shape) <= maxdim]
    if not ids:
        print("no small cliparts"); return
    hs = max(clips[i]["alpha"].shape[0] for i in ids)
    ws = max(clips[i]["alpha"].shape[1] for i in ids)
    M = len(ids)
    rgb = np.zeros((M, hs, ws, 3), dtype=np.float32)
    alpha = np.zeros((M, hs, ws), dtype=bool)
    means = np.zeros((M, 3))
    for k, i in enumerate(ids):
        c = clips[i]
        ah, aw = c["alpha"].shape
        rgb[k, :ah, :aw] = c["rgb"]
        alpha[k, :ah, :aw] = c["alpha"]
        means[k] = c["rgb"][c["alpha"]].mean(axis=0)
    print(f"stack: {M} cliparts up to {hs}x{ws}")

    def gain_at(r0, c0):
        a1 = min(hs, h - r0); b1 = min(ws, w - c0)
        ar0 = max(0, -r0); ac0 = max(0, -c0)
        if a1 <= ar0 or b1 <= ac0:
            return None, -1.0
        tgt = np.zeros((hs, ws, 3), dtype=np.float32)
        cur = np.zeros((hs, ws, 3), dtype=np.float32)
        inb = np.zeros((hs, ws), dtype=bool)
        tgt[ar0:a1, ac0:b1] = target[r0 + ar0 : r0 + a1, c0 + ac0 : c0 + b1]
        cur[ar0:a1, ac0:b1] = canvas[r0 + ar0 : r0 + a1, c0 + ac0 : c0 + b1]
        inb[ar0:a1, ac0:b1] = True
        cur_err = ((cur - tgt) ** 2).sum(axis=2)
        tm = tgt[inb].mean(axis=0)
        cand = np.argsort(((means - tm) ** 2).sum(axis=1))[:topk]
        diff = ((rgb[cand] - tgt[None]) ** 2).sum(axis=3)
        gain = np.where(alpha[cand] & inb[None], cur_err[None] - diff, 0.0).sum(axis=(1, 2))
        j = int(np.argmax(gain))
        return int(ids[cand[j]]) + 1, float(gain[j])

    def draw(pic, r, c):
        clip = clips[pic - 1]
        ch, cw = clip["alpha"].shape
        r0, r1 = max(r, 0), min(r + ch, h)
        c0, c1 = max(c, 0), min(c + cw, w)
        if r0 < r1 and c0 < c1:
            a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
            canvas[r0:r1, c0:c1][a] = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c][a]

    B = 16
    err = ((canvas.astype(np.float32) - target) ** 2).sum(axis=2)
    hh = h // B * B; ww = w // B * B
    eb = err[:hh, :ww].reshape(hh // B, B, ww // B, B).sum(axis=(1, 3))
    t0 = time.time()
    placed = 0
    budget = n_budget - len(placements)
    print(f"budget for detail: {budget}")
    while placed < budget:
        y, x = np.unravel_index(np.argmax(eb), eb.shape)
        if eb[y, x] <= 0:
            break
        cy, cx = y * B + B // 2, x * B + B // 2
        base_r, base_c = cy - hs // 2, cx - ws // 2
        pic, g0 = gain_at(base_r, base_c)
        best = (g0, pic, base_r, base_c)
        if jitter:
            for dr, dc in ((-jitter, -jitter), (-jitter, jitter), (jitter, -jitter), (jitter, jitter)):
                p2, g2 = gain_at(base_r + dr, base_c + dc)
                if g2 > best[0]:
                    best = (g2, p2, base_r + dr, base_c + dc)
        g0, pic, r0, c0 = best
        if pic is None or g0 <= 0:
            eb[y, x] = 0.0
            continue
        placements.append((pic, r0, c0))
        draw(pic, r0, c0)
        placed += 1
        rr0, cc0 = max(r0, 0), max(c0, 0)
        rr1 = min(r0 + hs, h); cc1 = min(c0 + ws, w)
        err[rr0:rr1, cc0:cc1] = ((canvas[rr0:rr1, cc0:cc1].astype(np.float32) - target[rr0:rr1, cc0:cc1]) ** 2).sum(axis=2)
        for by in range(rr0 // B, min((rr1 + B - 1) // B, hh // B)):
            for bx in range(cc0 // B, min((cc1 + B - 1) // B, ww // B)):
                eb[by, bx] = err[by * B : (by + 1) * B, bx * B : (bx + 1) * B].sum()
        if placed % 500 == 0:
            print(f"detail {placed}/{budget} err={mcc_a.score(target.astype(np.int16), canvas):,} ({time.time()-t0:.0f}s)", flush=True)
            save(path, placements)
    save(path, placements)
    print(f"done: placed {placed}, total {len(placements)}, err={mcc_a.score(target.astype(np.int16), canvas):,}")


def save(path, placements):
    with open(path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")


if __name__ == "__main__":
    main(int(sys.argv[1]),
         maxdim=int(sys.argv[2]) if len(sys.argv) > 2 else 260,
         topk=int(sys.argv[3]) if len(sys.argv) > 3 else 40,
         jitter=int(sys.argv[4]) if len(sys.argv) > 4 else 8,
         path=sys.argv[5] if len(sys.argv) > 5 else None)
