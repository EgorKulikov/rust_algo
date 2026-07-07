"""HODL API client: persistent connections, shared rate limiting, 429/5xx backoff."""

import json
import os
import threading
import time

import requests

BASE = "https://hodl.midnightcodecup.org"
_KEY_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "api-key.txt")
TOKEN = open(_KEY_PATH).read().strip()

_session = requests.Session()
_adapter = requests.adapters.HTTPAdapter(pool_connections=16, pool_maxsize=16)
_session.mount("https://", _adapter)
_session.headers["Authorization"] = "Bearer " + TOKEN


class RateLimiter:
    """Token bucket; server allows 10 req/s per team, we budget below that."""

    def __init__(self, rate=5.0, burst=5.0):
        self.rate = rate
        self.max_rate = 8.0
        self.min_rate = 2.0
        self.capacity = burst
        self.tokens = burst
        self.ts = time.monotonic()
        self.lock = threading.Lock()
        self.last_429 = 0.0

    def on_429(self):
        with self.lock:
            now = time.monotonic()
            if now - self.last_429 > 2.0:
                self.last_429 = now
                self.rate = max(self.min_rate, self.rate * 0.7)
                self.tokens = 0.0

    def on_ok(self):
        with self.lock:
            now = time.monotonic()
            if now - self.last_429 > 60.0:
                self.rate = min(self.max_rate, self.rate + 0.002)

    def acquire(self):
        while True:
            with self.lock:
                now = time.monotonic()
                self.tokens = min(self.capacity, self.tokens + (now - self.ts) * self.rate)
                self.ts = now
                if self.tokens >= 1.0:
                    self.tokens -= 1.0
                    return
                wait = (1.0 - self.tokens) / self.rate
            time.sleep(wait)


LIMITER = RateLimiter()


class ApiError(Exception):
    def __init__(self, code, msg):
        super().__init__(f"HTTP {code}: {msg}")
        self.code = code


def request(path, body=None, tries=5, idempotent=True):
    url = BASE + path
    for attempt in range(tries):
        LIMITER.acquire()
        try:
            if body is not None:
                resp = _session.post(url, json=body, timeout=25)
            else:
                resp = _session.get(url, timeout=25)
        except Exception:
            if not idempotent:
                # trade may have executed server-side; caller must resync, not retry
                raise ApiError(0, "timeout/network on non-idempotent request")
            if attempt == tries - 1:
                raise
            time.sleep(0.3 * (attempt + 1))
            continue
        if resp.status_code == 429:
            LIMITER.on_429()
            try:
                retry = float(resp.headers.get("Retry-After") or 1)
            except ValueError:
                retry = 1.0
            time.sleep(min(retry, 5.0))
            continue
        LIMITER.on_ok()
        if resp.status_code >= 500:
            time.sleep(0.4 * (attempt + 1))
            continue
        if resp.status_code >= 400:
            try:
                detail = resp.json().get("error", "")
            except Exception:
                detail = resp.text[:200]
            raise ApiError(resp.status_code, detail)
        return resp.json()
    raise ApiError(429, "retries exhausted")


def state():
    return request("/api/state")


def prices():
    return request("/api/stocks/prices")


def full_history():
    return request("/api/stocks/history")


def my_trades():
    return request("/api/trades")


def trade(action, stock_id, amount):
    return request("/api/" + action, {"stock_id": stock_id, "amount": int(amount)},
                   idempotent=False)
