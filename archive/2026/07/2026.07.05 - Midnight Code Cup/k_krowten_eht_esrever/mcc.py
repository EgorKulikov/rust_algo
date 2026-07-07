"""Client + image utils for task K (Krowten eht Esrever / "Reverse the Network").

We POST a strictly black/white 256x256 PNG to a beast endpoint and get back
{"score": float 0..100, "key": str|null}. Higher score = beast likes it more.
When the beast is pleased enough, key becomes non-null (600 pts, submit it).

Rate limit: <= 1 call/sec per endpoint per team. We enforce a safety margin
and handle 429 with backoff. Every query is appended to a jsonl log; the best
image and score per endpoint are tracked on disk so we can resume.
"""
import io
import json
import os
import sys
import time
import hashlib
import threading

import numpy as np
from PIL import Image
import requests

HERE = os.path.dirname(os.path.abspath(__file__))

ENDPOINTS = {
    "a": "https://krowten.midnightcodecup.org/task-a",
    "b": "https://krowten.midnightcodecup.org/task-b",
}

N = 256  # image side


def load_token():
    tok = os.environ.get("MCC_TOKEN")
    if tok:
        return tok.strip()
    for fname in ("token.txt", ".token"):
        p = os.path.join(HERE, fname)
        if os.path.exists(p):
            with open(p) as f:
                return f.read().strip()
    raise SystemExit(
        "No API token. Set MCC_TOKEN env var or put it in tasks/k/token.txt"
    )


def to_bw_uint8(arr):
    """arr: bool or {0,1} or {0,255} HxW -> uint8 HxWx3 strictly 0/255."""
    a = np.asarray(arr)
    if a.dtype == bool:
        m = a
    else:
        m = a > (a.max() / 2.0 if a.max() > 1 else 0.5)
    px = np.where(m, np.uint8(255), np.uint8(0))
    return np.repeat(px[:, :, None], 3, axis=2)


def encode_png(arr):
    """arr -> raw PNG bytes, strictly black/white, grayscale (mode 'L').

    The beast rejects RGB ("expected a grayscale PNG"); it wants a single-channel
    image whose pixels are exactly 0 or 255.
    """
    gray = to_bw_uint8(arr)[:, :, 0]
    img = Image.fromarray(gray, "L")
    buf = io.BytesIO()
    img.save(buf, format="PNG")
    return buf.getvalue()


def img_hash(arr):
    return hashlib.sha1(to_bw_uint8(arr).tobytes()).hexdigest()[:12]


class Beast:
    def __init__(self, name, token=None, min_interval=1.15):
        self.name = name
        self.url = ENDPOINTS[name]
        self.token = token or load_token()
        self.min_interval = min_interval
        self._last = 0.0
        self._lock = threading.Lock()
        self.log_path = os.path.join(HERE, f"log_{name}.jsonl")
        self.best_score = -1.0
        self.best_arr = None
        self.key = None
        self.n_queries = 0
        self._load_state()

    def _load_state(self):
        if os.path.exists(self.log_path):
            with open(self.log_path) as f:
                for line in f:
                    try:
                        rec = json.loads(line)
                    except Exception:
                        continue
                    self.n_queries += 1
                    if rec.get("key"):
                        self.key = rec["key"]
                    if rec.get("score", -1) > self.best_score:
                        self.best_score = rec["score"]
        bp = os.path.join(HERE, f"best_{self.name}.png")
        if os.path.exists(bp):
            self.best_arr = (np.asarray(Image.open(bp).convert("L")) > 127)

    def _throttle(self):
        dt = time.time() - self._last
        if dt < self.min_interval:
            time.sleep(self.min_interval - dt)

    def query(self, arr, tag=""):
        """Send image, return (score, key). Retries on 429/5xx."""
        png = encode_png(arr)
        h = img_hash(arr)
        with self._lock:
            while True:
                self._throttle()
                self._last = time.time()
                try:
                    r = requests.post(
                        self.url,
                        headers={
                            "Authorization": f"Bearer {self.token}",
                            "Content-Type": "image/png",
                        },
                        data=png,
                        timeout=30,
                    )
                except requests.RequestException as e:
                    print(f"[{self.name}] net error {e}; retry in 2s", file=sys.stderr)
                    time.sleep(2)
                    continue
                if r.status_code == 429:
                    time.sleep(1.5)
                    continue
                if r.status_code >= 500:
                    print(f"[{self.name}] {r.status_code}; retry", file=sys.stderr)
                    time.sleep(2)
                    continue
                if r.status_code != 200:
                    raise SystemExit(f"[{self.name}] HTTP {r.status_code}: {r.text[:300]}")
                data = r.json()
                break
        score = float(data.get("score", 0.0))
        key = data.get("key")
        self.n_queries += 1
        rec = {"t": round(time.time(), 2), "score": score, "key": key,
               "hash": h, "tag": tag}
        with open(self.log_path, "a") as f:
            f.write(json.dumps(rec) + "\n")
        if key and not self.key:
            self.key = key
            with open(os.path.join(HERE, f"KEY_{self.name}.txt"), "w") as f:
                f.write(key + "\n")
            print(f"\n*** [{self.name}] KEY FOUND: {key}  (score {score}) ***\n")
        if score > self.best_score:
            self.best_score = score
            self.best_arr = to_bw_uint8(arr)[:, :, 0] > 127
            Image.fromarray(to_bw_uint8(arr), "RGB").save(
                os.path.join(HERE, f"best_{self.name}.png"))
        print(f"[{self.name}] q#{self.n_queries} score={score:6.3f} "
              f"best={self.best_score:6.3f} tag={tag}")
        return score, key
