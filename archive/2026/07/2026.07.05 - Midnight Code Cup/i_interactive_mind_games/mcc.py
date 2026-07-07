"""Protocol client for MCC problem I "Interactive Mind Games".

Three games behind one REST root; bearer auth via checksystem token.
Rate limit: 30 requests / 60 s per game per token, tracked independently.
"""
import json
import time
import urllib.error
import urllib.request
from pathlib import Path

TASK_DIR = Path(__file__).resolve().parent
API = "https://mind-games.midnightcodecup.org/api/v1/games"
KEY = (TASK_DIR / ".." / "api-key.txt").read_text().strip()

US = "unique-smallest"
BLOTTO = "blotto"
DOOR = "door-rush"
GAMES = [US, BLOTTO, DOOR]


class ApiError(Exception):
    def __init__(self, code, body):
        super().__init__(f"HTTP {code}: {body[:300]}")
        self.code = code
        self.body = body


def _request(game, path, payload=None, auth=True, timeout=10, tries=4):
    url = f"{API}/{game}/{path}"
    headers = {}
    if auth:
        headers["Authorization"] = f"Bearer {KEY}"
    data = None
    if payload is not None:
        data = json.dumps(payload).encode()
        headers["Content-Type"] = "application/json"
    last = None
    for attempt in range(tries):
        try:
            req = urllib.request.Request(url, data=data, headers=headers)
            with urllib.request.urlopen(req, timeout=timeout) as resp:
                return json.loads(resp.read())
        except urllib.error.HTTPError as e:
            body = e.read().decode(errors="replace")
            last = ApiError(e.code, body)
            if e.code == 429 or e.code >= 500:
                time.sleep(2.0 * (attempt + 1))
                continue
            raise last from None
        except (urllib.error.URLError, TimeoutError, OSError) as e:
            last = e
            time.sleep(1.0 * (attempt + 1))
    raise last


def config(game):
    return _request(game, "config", auth=False)


def status(game):
    return _request(game, "status", auth=False)


def last_round(game):
    return _request(game, "last-round")


def submit_number(number):
    return _request(US, "move", {"number": int(number)})


def submit_allocation(allocation):
    allocation = [int(x) for x in allocation]
    assert len(allocation) == 5 and sum(allocation) == 100 and min(allocation) >= 0
    return _request(BLOTTO, "move", {"allocation": allocation})


def submit_entrance(entrance, mode):
    assert 0 <= entrance <= 5 and mode in ("rush", "flex")
    return _request(DOOR, "move", {"entrance": int(entrance), "mode": mode})


def submit_room(room):
    assert 0 <= room <= 5
    return _request(DOOR, "move", {"room": int(room)})


def entrance_counts():
    return _request(DOOR, "entrance-counts")["entrance_counts"]
