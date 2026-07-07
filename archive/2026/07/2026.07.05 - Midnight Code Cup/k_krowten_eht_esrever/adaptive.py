"""Adaptive coarse-to-fine quadtree optimizer on the committed image.

Model (empirical): score ~ max(0, base + sum of per-pixel target weights over
white pixels). So the optimum paints white exactly where the weight is positive.
We greedily grow the committed white set: test a black region flipped to white;
if it improves, commit the whole block at once (net-positive region); otherwise
subdivide and recurse down to 1px. Coarse levels commit big white areas cheaply
and rule out big black areas by only subdividing where there is signal.

Runs forever over refinement rounds until a key drops. Resumes from best_<name>.
"""
import sys
import numpy as np
import mcc

N = mcc.N


def optimize(beast, min_size=1, start_size=64, prune_drop=None):
    cur = beast.best_arr.copy() if beast.best_arr is not None else np.zeros((N, N), bool)
    cur_score, _ = beast.query(cur, tag="adapt:seed")
    # process regions coarse -> fine via an explicit stack
    stack = []
    step = start_size
    for y in range(0, N, step):
        for x in range(0, N, step):
            stack.append((y, x, step))
    committed = 0
    tested = 0
    while stack:
        if beast.key:
            return cur
        y, x, s = stack.pop()
        blk = cur[y:y + s, x:x + s]
        if blk.all():
            continue  # already all white here
        trial = cur.copy()
        trial[y:y + s, x:x + s] = True
        sc, _ = beast.query(trial, tag=f"adapt:{s}:{y},{x}")
        tested += 1
        d = sc - cur_score
        if d > 1e-9:
            cur = trial
            cur_score = sc
            committed += 1
        elif s > min_size:
            # subdivide unless the region is strongly negative (solid black)
            if prune_drop is not None and d < -prune_drop:
                continue
            h = s // 2
            for dy in (0, h):
                for dx in (0, h):
                    stack.append((y + dy, x + dx, h))
        if tested % 100 == 0:
            print(f"[{beast.name}] adapt tested={tested} committed={committed} "
                  f"score={cur_score:.3f} stack={len(stack)}", flush=True)
    print(f"[{beast.name}] adapt round done score={cur_score:.4f} "
          f"tested={tested} committed={committed}", flush=True)
    return cur


if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "a"
    minsz = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    beast = mcc.Beast(name)
    print(f"[{name}] adaptive start best={beast.best_score:.4f} key={beast.key}", flush=True)
    while not beast.key:
        optimize(beast, min_size=minsz)
        print(f"[{name}] loop best={beast.best_score:.4f}", flush=True)
    print(f"[{name}] KEY {beast.key}")
