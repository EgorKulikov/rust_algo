"""Shared harness for MCC task A (Arcimboldo's Style).

Semantics per check.cpp: canvas of target size, all three RGB channels start
at -255; drawing a clipart at (row, col) overwrites RGB wherever alpha != 0
(pixels outside canvas ignored). Error = sum over all pixels of squared RGB
channel diffs vs target. Output NN.out: M, then M lines "pic r c" (pic
1-based clipart id), later lines drawn on top.
"""
import json
import os
import sys
import urllib.request
import uuid

import numpy as np
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "data")
OUT = os.path.join(HERE, "out")
CACHE = os.path.join(HERE, "cache")
API = "https://finals.midnightcodecup.org/api/v1"


def api_key():
    with open(os.path.join(HERE, "..", "api-key.txt")) as f:
        return f.read().strip()


def load_input(dd):
    with open(os.path.join(DATA, f"test{dd:02d}.in")) as f:
        lines = f.read().splitlines()
    return {"n": int(lines[0]), "target": lines[1], "collection": lines[2]}


def load_target(name):
    """HxWx3 int16 RGB."""
    im = Image.open(os.path.join(DATA, name)).convert("RGB")
    return np.asarray(im, dtype=np.int16)


def load_collection(name):
    """Returns list (1-indexed via [id-1]) of dicts: rgb uint8 HxWx3, alpha bool HxW."""
    cache = os.path.join(CACHE, f"{name}.npz")
    files = sorted(os.listdir(os.path.join(DATA, name)))
    if os.path.exists(cache):
        z = np.load(cache, allow_pickle=True)
        return list(z["clips"])
    clips = []
    for fn in files:
        arr = np.asarray(Image.open(os.path.join(DATA, name, fn)))
        assert arr.shape[2] == 4, fn
        clips.append({"rgb": arr[:, :, :3].copy(), "alpha": arr[:, :, 3] != 0})
    os.makedirs(CACHE, exist_ok=True)
    np.savez_compressed(cache, clips=np.array(clips, dtype=object))
    return clips


def render(placements, shape, clips):
    """placements: iterable of (pic, r, c) 1-based pic. Returns HxWx3 int16 canvas."""
    h, w = shape[:2]
    canvas = np.full((h, w, 3), -255, dtype=np.int16)
    for pic, r, c in placements:
        clip = clips[pic - 1]
        ch, cw = clip["alpha"].shape
        r0, r1 = max(r, 0), min(r + ch, h)
        c0, c1 = max(c, 0), min(c + cw, w)
        if r0 >= r1 or c0 >= c1:
            continue
        sub_a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
        canvas[r0:r1, c0:c1][sub_a] = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c][sub_a]
    return canvas


def score(target, canvas):
    d = (target.astype(np.int64) - canvas.astype(np.int64)) ** 2
    return int(d.sum())


def score_file(dd, path=None):
    info = load_input(dd)
    target = load_target(info["target"])
    clips = load_collection(info["collection"])
    placements = read_out(path or os.path.join(OUT, f"{dd:02d}.out"))
    assert len(placements) <= info["n"], f"{len(placements)} > {info['n']}"
    return score(target, render(placements, target.shape, clips))


def write_out(dd, placements):
    os.makedirs(OUT, exist_ok=True)
    path = os.path.join(OUT, f"{dd:02d}.out")
    with open(path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    return path


def read_out(path):
    with open(path) as f:
        toks = f.read().split()
    m = int(toks[0])
    vals = list(map(int, toks[1 : 1 + 3 * m]))
    return [tuple(vals[3 * i : 3 * i + 3]) for i in range(m)]


def submit(paths):
    """Submit one or more NN.out files. Returns response json."""
    boundary = uuid.uuid4().hex
    body = b""
    for p in paths:
        name = os.path.basename(p)
        with open(p, "rb") as f:
            content = f.read()
        body += (
            f"--{boundary}\r\n"
            f'Content-Disposition: form-data; name="files"; filename="{name}"\r\n'
            f"Content-Type: application/octet-stream\r\n\r\n"
        ).encode() + content + b"\r\n"
    body += f"--{boundary}--\r\n".encode()
    req = urllib.request.Request(
        f"{API}/problems/A/submit-outputs",
        data=body,
        headers={
            "Authorization": f"Bearer {api_key()}",
            "Content-Type": f"multipart/form-data; boundary={boundary}",
        },
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.loads(resp.read())


def api_get(path):
    req = urllib.request.Request(API + path, headers={"Authorization": f"Bearer {api_key()}"})
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.loads(resp.read())


if __name__ == "__main__":
    if sys.argv[1] == "score":
        for dd in map(int, sys.argv[2:]):
            print(dd, f"{score_file(dd):,}")
    elif sys.argv[1] == "submit":
        print(json.dumps(submit([os.path.join(OUT, f"{int(a):02d}.out") for a in sys.argv[2:]]), indent=1))
