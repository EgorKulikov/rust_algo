#!/usr/bin/env python3
"""Ground-truth check on real data: split each corrupted chunk into two
halves; each chunk's halves are truly adjacent. Measure how many of these
known adjacencies the solver reproduces.

Usage: validate.py work/SS-RR.in [splits]
"""
import subprocess
import sys
from pathlib import Path

TASK_DIR = Path(__file__).resolve().parent
SOLVER = TASK_DIR / "../../target/release/b"


def main() -> None:
    path = Path(sys.argv[1])
    splits = int(sys.argv[2]) if len(sys.argv) > 2 else 2
    toks = path.read_text().split()
    n, size = int(toks[0]), int(toks[1])
    m = n * splits
    inp = f"{m} {size}\n" + " ".join(toks[2:]) + "\n"
    res = subprocess.run([str(SOLVER)], input=inp, capture_output=True, text=True, check=True)
    perm = [int(t) for t in res.stdout.split()]
    assert sorted(perm) == list(range(m))
    pos = {c: j for j, c in enumerate(perm)}
    hits = sum(
        1
        for c in range(m)
        if c % splits != splits - 1 and pos[c] + 1 < m and perm[pos[c] + 1] == c + 1
    )
    total = n * (splits - 1)
    print(f"{path.name} n={n} splits={splits}: recovered {hits}/{total} known adjacencies")


if __name__ == "__main__":
    main()
