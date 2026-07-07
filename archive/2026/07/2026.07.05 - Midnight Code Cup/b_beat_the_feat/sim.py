#!/usr/bin/env python3
"""Synthetic round-trip test: shuffle a known audio, run the solver, score it.

Usage: sim.py ORIGINAL.in RECORD_NUM [seed]
ORIGINAL.in holds the *original* (unshuffled) audio in solver .in format
(first two tokens ignored for n; samples follow).
"""
import random
import subprocess
import sys
from collections import Counter
from pathlib import Path

TASK_DIR = Path(__file__).resolve().parent
SOLVER = TASK_DIR / "../../target/release/b"
SIZE = 1806336


def load_samples(path: Path) -> list[int]:
    toks = path.read_text().split()
    return [int(t) for t in toks[2:]]


def chunk_ids(samples: list[int], n: int) -> list[int]:
    cs = len(samples) // n
    seen: dict[tuple, int] = {}
    ids = []
    for i in range(n):
        key = tuple(samples[i * cs : (i + 1) * cs])
        ids.append(seen.setdefault(key, len(seen)))
    return ids


def lccs(a: list[int], b: list[int]) -> int:
    # longest common contiguous block, binary search on length + rolling hash
    mod, base = (1 << 61) - 1, 1_000_003

    def ok(length: int) -> bool:
        if length == 0:
            return True
        pw = pow(base, length - 1, mod)

        def hashes(s):
            out = set()
            h = 0
            for i, x in enumerate(s):
                if i >= length:
                    h = (h - s[i - length] * pw) % mod
                h = (h * base + x) % mod
                if i >= length - 1:
                    out.add(h)
            return out

        return not hashes(a).isdisjoint(hashes(b))

    lo, hi = 0, min(len(a), len(b))
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if ok(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo


def score(orig_ids: list[int], out_ids: list[int]) -> tuple[float, int, int]:
    n = len(orig_ids)
    l = lccs(orig_ids, out_ids)
    pairs = Counter(zip(orig_ids, orig_ids[1:]))
    pairs.subtract(Counter(zip(out_ids, out_ids[1:])))
    ncp = (n - 1) - sum(c for c in pairs.values() if c > 0)
    return 0.5 * (l - 1) / (n - 1) + 0.5 * ncp / (n - 1), l, ncp


def main() -> None:
    orig_path, r = Path(sys.argv[1]), int(sys.argv[2])
    seed = int(sys.argv[3]) if len(sys.argv) > 3 else 42
    n = 3 * 2 ** (13 - r)
    orig = load_samples(orig_path)
    assert len(orig) == SIZE
    cs = SIZE // n

    rng = random.Random(seed)
    hidden = list(range(n))
    while True:
        rng.shuffle(hidden)
        if hidden != list(range(n)):
            break
    # hidden[j] = original position of the chunk placed at corrupted position j
    shuffled = []
    for j in range(n):
        shuffled.extend(orig[hidden[j] * cs : (hidden[j] + 1) * cs])

    inp = f"{n} {SIZE}\n" + " ".join(map(str, shuffled)) + "\n"
    res = subprocess.run([str(SOLVER)], input=inp, capture_output=True, text=True, check=True)
    perm = [int(t) for t in res.stdout.split()]
    assert sorted(perm) == list(range(n)), "not a permutation"

    restored = []
    for j in range(n):
        restored.extend(shuffled[perm[j] * cs : (perm[j] + 1) * cs])

    orig_ids = chunk_ids(orig, n)
    out_ids = chunk_ids(orig + restored, n * 2)[n:]  # share id space
    orig_ids2 = chunk_ids(orig + restored, n * 2)[:n]
    s, l, ncp = score(orig_ids2, out_ids)
    exact = restored == orig
    print(f"record {r} (n={n}): score={s:.4f} lccs={l}/{n} ncp={ncp}/{n - 1} exact={exact}")


if __name__ == "__main__":
    main()
