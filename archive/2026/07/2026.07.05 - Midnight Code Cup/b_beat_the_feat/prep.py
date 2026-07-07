#!/usr/bin/env python3
"""Extract a record's WAV and convert to .in for the solver.

Usage: prep.py SS-RR PASSWORD [workdir]
Writes <workdir>/SS-RR.in with: n size samples...
"""
import struct
import subprocess
import sys
from pathlib import Path

TASK_DIR = Path(__file__).resolve().parent
SIZE = 1806336


def wav_samples(path: Path) -> list[int]:
    data = path.read_bytes()
    assert data[:4] == b"RIFF" and data[8:12] == b"WAVE", "not a WAV"
    # find data chunk
    off = 12
    while off < len(data):
        cid, clen = data[off : off + 4], struct.unpack("<I", data[off + 4 : off + 8])[0]
        if cid == b"data":
            body = data[off + 8 : off + 8 + clen]
            return list(struct.unpack(f"<{clen // 2}h", body))
        off += 8 + clen
    raise AssertionError("no data chunk")


def main() -> None:
    rec, password = sys.argv[1], sys.argv[2]
    workdir = Path(sys.argv[3]) if len(sys.argv) > 3 else TASK_DIR / "work"
    workdir.mkdir(parents=True, exist_ok=True)
    r = int(rec.split("-")[1])
    n = 3 * 2 ** (13 - r)
    subprocess.run(
        ["7z", "x", "-y", f"-p{password}", f"-o{workdir}", str(TASK_DIR / f"{rec}.zip")],
        check=True,
        capture_output=True,
    )
    samples = wav_samples(workdir / f"{rec}.wav")
    assert len(samples) == SIZE, f"expected {SIZE} samples, got {len(samples)}"
    with open(workdir / f"{rec}.in", "w") as f:
        f.write(f"{n} {SIZE}\n")
        f.write(" ".join(map(str, samples)))
        f.write("\n")
    print(f"wrote {workdir / f'{rec}.in'} (n={n})")


if __name__ == "__main__":
    main()
