"""Driver for task K black-box optimization.

Usage:
  python run.py probe [a|b|both]      # run the probe battery
  python run.py climb [a|b|both]      # coarse-to-fine block hill climb from best
  python run.py both                  # probe then climb, both endpoints, threaded

Strategy: the beast is a network returning a scalar "how much I like this image"
score in 0..100; when high enough it yields the key. We first probe a diverse
battery to find which image family the beast responds to, then hill-climb over a
grid of pixel blocks (coarse -> fine), keeping every flip that improves the
score. Two endpoints run in parallel threads (independent 1/sec budgets).
"""
import sys
import threading

import numpy as np

import mcc
import images

N = mcc.N


def run_probe(beast, stop_at=99.0):
    best_tag, best_arr = None, None
    for tag, arr in images.probe_images():
        if beast.key:
            return beast.best_arr
        s, k = beast.query(arr, tag=f"probe:{tag}")
        if s == beast.best_score:
            best_tag, best_arr = tag, arr
        if s >= stop_at:
            break
    print(f"[{beast.name}] probe done, best={beast.best_score:.3f} ({best_tag})")
    return beast.best_arr


def climb(beast, seed=None, grids=(8, 16, 32, 64), rounds_per_grid=3):
    """Coarse-to-fine block hill climb. Flip one block, keep if score improves."""
    if seed is None:
        seed = beast.best_arr
    if seed is None:
        seed = np.zeros((N, N), bool)
    cur = seed.copy()
    cur_score = beast.best_score
    rng = np.random.default_rng(7)
    for g in grids:
        bs = N // g
        for rnd in range(rounds_per_grid):
            order = [(i, j) for i in range(g) for j in range(g)]
            rng.shuffle(order)
            improved = 0
            for (i, j) in order:
                if beast.key:
                    return cur
                trial = cur.copy()
                y0, x0 = i * bs, j * bs
                trial[y0:y0 + bs, x0:x0 + bs] ^= True
                s, k = beast.query(trial, tag=f"climb:g{g}:{i},{j}")
                if s > cur_score + 1e-9:
                    cur, cur_score = trial, s
                    improved += 1
            print(f"[{beast.name}] grid{g} round{rnd}: score={cur_score:.3f} "
                  f"improved={improved}")
            if improved == 0:
                break
    return cur


def drive(name, do_probe=True, do_climb=True):
    beast = mcc.Beast(name)
    print(f"[{name}] resume: {beast.n_queries} queries, best={beast.best_score:.3f}, "
          f"key={beast.key}")
    if beast.key:
        print(f"[{name}] already solved: {beast.key}")
        return
    seed = None
    if do_probe:
        seed = run_probe(beast)
    if do_climb and not beast.key:
        climb(beast, seed=seed)
    print(f"[{name}] final best={beast.best_score:.3f} key={beast.key}")


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else "both"
    which = sys.argv[2] if len(sys.argv) > 2 else "both"
    do_probe = cmd in ("probe", "both")
    do_climb = cmd in ("climb", "both")
    names = ["a", "b"] if which == "both" else [which]
    threads = [threading.Thread(target=drive, args=(n, do_probe, do_climb))
               for n in names]
    for t in threads:
        t.start()
    for t in threads:
        t.join()


if __name__ == "__main__":
    main()
