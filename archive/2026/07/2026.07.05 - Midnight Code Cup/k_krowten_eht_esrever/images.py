"""Candidate image generators for the probe phase.

Each generator yields (tag, bool-array HxH). The probe phase sends a diverse
battery so we can see what each beast responds to, then the optimizer refines
around the best-scoring family.
"""
import os
import numpy as np
from PIL import Image, ImageDraw, ImageFont

HERE = os.path.dirname(os.path.abspath(__file__))
N = 256
_yy, _xx = np.mgrid[0:N, 0:N]


def _try_font(size):
    for path in (
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
    ):
        if os.path.exists(path):
            try:
                return ImageFont.truetype(path, size)
            except Exception:
                pass
    return ImageFont.load_default()


def text_img(text, size=80, invert=False):
    img = Image.new("L", (N, N), 0)
    d = ImageDraw.Draw(img)
    font = _try_font(size)
    # wrap into lines that fit
    lines = text.split("\n")
    bb = [d.textbbox((0, 0), ln, font=font) for ln in lines]
    hs = [b[3] - b[1] for b in bb]
    total = sum(hs) + 6 * (len(lines) - 1)
    y = (N - total) // 2
    for ln, b in zip(lines, bb):
        w = b[2] - b[0]
        d.text(((N - w) // 2 - b[0], y - b[1]), ln, fill=255, font=font)
        y += (b[3] - b[1]) + 6
    a = np.asarray(img) > 127
    return ~a if invert else a


def probe_images():
    out = []
    out.append(("allblack", np.zeros((N, N), bool)))
    out.append(("allwhite", np.ones((N, N), bool)))

    for s in (1, 2, 4, 8, 16, 32, 64):
        out.append((f"checker{s}", (((_xx // s) + (_yy // s)) % 2 == 0)))
    for p in (2, 4, 8, 16, 32, 64):
        out.append((f"vstripe{p}", ((_xx // (p // 2 if p > 1 else 1)) % 2 == 0)))
        out.append((f"hstripe{p}", ((_yy // (p // 2 if p > 1 else 1)) % 2 == 0)))
        out.append((f"diag{p}", (((_xx + _yy) // (p // 2 if p > 1 else 1)) % 2 == 0)))

    rng = np.random.default_rng(12345)
    for d in (10, 25, 50, 75, 90):
        out.append((f"noise{d}", rng.random((N, N)) < d / 100.0))

    cx = cy = N // 2
    r = np.sqrt((_xx - cx) ** 2 + (_yy - cy) ** 2)
    for rad in (32, 64, 96, 120):
        out.append((f"disc{rad}", r < rad))
        out.append((f"ring{rad}", np.abs(r - rad) < 12))
    for half in (32, 64, 96):
        sq = (np.abs(_xx - cx) < half) & (np.abs(_yy - cy) < half)
        out.append((f"square{half}", sq))

    out.append(("rings", (r.astype(int) // 12) % 2 == 0))
    out.append(("bayer", _bayer()))

    words = ["REVERSE", "NETWORK", "KEY", "PLEASE", "OPENAI", "HELLO",
             "CAT", "DOG", "YES", "MCC", "A", "B", "1", "0", "GPT",
             "REVERSE\nNETWORK"]
    for w in words:
        out.append((f"txt_{w.replace(chr(10),'_')}", text_img(w)))

    # OpenAI logo assets shipped with the statement page
    for fn in ("openai_black.png", "openai_white.png"):
        p = os.path.join(HERE, "K. Krowten eht Esrever — Midnight Code Cup_files", fn)
        if os.path.exists(p):
            im = Image.open(p).convert("L").resize((N, N))
            out.append((fn[:-4], np.asarray(im) > 127))
            out.append((fn[:-4] + "_inv", np.asarray(im) <= 127))

    return out


def _bayer():
    M = np.array([[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]])
    thr = np.tile(M, (N // 4, N // 4)) / 16.0
    grad = _xx / N
    return grad > thr
