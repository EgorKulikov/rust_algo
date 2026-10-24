#!/usr/bin/env python3
"""Client for the Cloudflight Coding Contest platform (codingcontest.org).

Flow: hub login (session cookie + XSRF) -> game token for a contest slug ->
per-contest game host, whose API takes the token in a plain `Authorization`
header plus `X-CCC-SLUG`, and only answers requests that look like they come
from the hub's iframe (Referer / Sec-Fetch headers).

Usage:
  ccc.py login                         # log in, store cookies in tasks/ccc/session/
  ccc.py contests                      # list current contests
  ccc.py training [start <game> [mode]] # active training sessions / past games; start one
  ccc.py info <contest-slug>           # contestant-info: levels, score, cooldowns
  ccc.py files <slug> <level>          # download level-<n>.zip (statement + inputs)
  ccc.py input <slug> <level> <file>   # print one input file
  ccc.py submit <slug> <level> <file> <path>   # upload an output file (file id like 1-small)
Slugs: a contest slug (42nd-classic-ccc-5872) or a training session name from `training`.
Credentials: creds.txt in the repo root with `login=<user>` and `pass=<password>` lines.
"""
import json, os, sys, http.cookiejar, urllib.request, urllib.parse, uuid, time

ROOT = os.path.dirname(os.path.abspath(__file__))
HUB = "https://codingcontest.org"
SESSION_DIR = os.path.join(ROOT, "session")
JAR = os.path.join(SESSION_DIR, "cookies.txt")
TOKENS = os.path.join(SESSION_DIR, "tokens.json")
DOWNLOADS = os.path.join(ROOT, "downloads")
os.makedirs(SESSION_DIR, exist_ok=True)
os.makedirs(DOWNLOADS, exist_ok=True)

cj = http.cookiejar.MozillaCookieJar(JAR)
if os.path.exists(JAR):
    cj.load(ignore_discard=True, ignore_expires=True)
opener = urllib.request.build_opener(urllib.request.HTTPCookieProcessor(cj))
UA = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0 Safari/537.36"


def xsrf():
    return next((c.value for c in cj if c.name == "XSRF-TOKEN"), "")


def hub(method, path, body=None, params=None):
    url = HUB + path + ("?" + urllib.parse.urlencode(params) if params else "")
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(url, data=data, method=method, headers={
        "Content-Type": "application/json", "Accept": "application/json",
        "X-XSRF-TOKEN": xsrf(), "User-Agent": UA, "Referer": HUB + "/"})
    try:
        with opener.open(req) as r:
            raw = r.read()
            cj.save(ignore_discard=True, ignore_expires=True)
            return r.status, (json.loads(raw) if raw.strip() else None)
    except urllib.error.HTTPError as e:
        raw = e.read()
        try:
            return e.code, json.loads(raw)
        except Exception:
            return e.code, raw[:500]


def read_creds():
    """creds.txt: `login=<user>` and `pass=<password>` lines (values may contain `=`)."""
    kv = {}
    for line in open(os.path.join(ROOT, "..", "..", "creds.txt")).read().splitlines():
        if "=" in line:
            k, v = line.split("=", 1)
            kv[k.strip().lower()] = v.strip()
    return kv["login"], kv["pass"]


def login():
    user, pw = read_creds()
    hub("GET", "/api/contests", params={"size": 1})  # obtain the XSRF cookie
    code, res = hub("POST", "/api/auth/login", {"username": user, "password": pw})
    if code != 200:
        sys.exit(f"login failed: {code} {res}")
    print("logged in as", res.get("username"), "roles", res.get("roles"))
    if "ROLE_PARTICIPANT" not in (res.get("roles") or []):
        code, res = hub("POST", "/api/auth/accept-participant-role")
        print("accept-participant-role:", code, res.get("roles") if isinstance(res, dict) else res)


def current_user():
    return hub("GET", "/api/auth/current-user")


def contests(all_pages=False):
    items, page = [], 0
    while True:
        code, d = hub("GET", "/api/contests", params={"page": page, "size": 50})
        items += d["content"]
        page += 1
        if page >= d["totalPages"] or not all_pages:
            break
    return items


def find_contest(slug):
    """A real contest by slug, or an active training session (slug starts with `training-`)."""
    code, c = hub("GET", f"/api/contests/{urllib.parse.quote(slug)}")
    if code == 200 and isinstance(c, dict) and c.get("gameBaseUrl"):
        return c
    code, sessions = hub("GET", "/api/training/active")
    for t in sessions or []:
        if t["contestName"] == slug:
            code, game = hub("GET", f"/api/training/{t['gameSlug']}")
            return {"slug": slug, "status": "TRAINING", "gameBaseUrl": game["baseUrl"], "gameName": game["name"]}
    if code == 200 and isinstance(c, dict):
        return c
    sys.exit(f"no contest or training session with slug {slug}")


def game_token(slug):
    """Cached per slug; the hub mints one per request, lifetime unknown."""
    cache = json.load(open(TOKENS)) if os.path.exists(TOKENS) else {}
    if slug in cache and time.time() - cache[slug]["at"] < 20 * 60:
        return cache[slug]["token"]
    code, res = hub("POST", "/api/game-token", {"contestSlug": slug})
    if code != 200:
        sys.exit(f"game-token failed: {code} {res} (not logged in or not registered for the contest?)")
    cache[slug] = {"token": res["token"], "at": time.time()}
    json.dump(cache, open(TOKENS, "w"))
    return res["token"]


class Game:
    def __init__(self, slug):
        self.contest = find_contest(slug)
        self.base = self.contest["gameBaseUrl"].rstrip("/")
        if not self.base:
            sys.exit(f"contest {slug} has no game host yet (status {self.contest['status']})")
        self.slug = slug
        self.token = game_token(slug)

    def headers(self, extra=None):
        h = {"Authorization": self.token, "X-CCC-SLUG": self.slug, "User-Agent": UA,
             "Referer": HUB + "/", "Origin": self.base,
             "Sec-Fetch-Dest": "empty", "Sec-Fetch-Mode": "cors", "Sec-Fetch-Site": "same-origin"}
        h.update(extra or {})
        return h

    def request(self, method, path, data=None, extra=None):
        req = urllib.request.Request(self.base + path, data=data, method=method, headers=self.headers(extra))
        try:
            with urllib.request.urlopen(req) as r:
                return r.status, r.headers.get("content-type", ""), r.read()
        except urllib.error.HTTPError as e:
            return e.code, e.headers.get("content-type", ""), e.read()

    def info(self):
        code, ct, raw = self.request("GET", "/api/contestant/contestant-info")
        if code != 200:
            sys.exit(f"contestant-info: {code} {raw[:300]}")
        return json.loads(raw)

    def level_files(self, level):
        code, ct, raw = self.request("GET", f"/api/contestant/level/{level}/files")
        if code != 200:
            sys.exit(f"level files: {code} {raw[:300]}")
        if "application/json" in ct:   # redirect to a signed URL
            url = json.loads(raw)["url"]
            raw = urllib.request.urlopen(url).read()
        path = os.path.join(DOWNLOADS, f"{self.slug}-level-{level}.zip")
        open(path, "wb").write(raw)
        return path

    def input_file(self, level, file_id):
        code, ct, raw = self.request("GET", f"/api/contestant/level/{level}/input/{file_id}")
        if code == 200 and "application/json" in ct:
            raw = urllib.request.urlopen(json.loads(raw)["url"]).read()
        elif code != 200:
            code, ct, raw = self.request("GET", f"/api/contestant/level/{level}/input/{file_id}?raw=true")
        return code, raw

    def submit(self, level, file_id, path):
        boundary = uuid.uuid4().hex
        name = os.path.basename(path)
        body = (f"--{boundary}\r\nContent-Disposition: form-data; name=\"solution\"; filename=\"{name}\"\r\n"
                f"Content-Type: text/plain\r\n\r\n").encode() + open(path, "rb").read() + f"\r\n--{boundary}--\r\n".encode()
        code, ct, raw = self.request("POST", f"/api/contestant/submit-{level}-{file_id}", data=body,
                                     extra={"Content-Type": f"multipart/form-data; boundary={boundary}"})
        try:
            return code, json.loads(raw)
        except Exception:
            return code, raw[:500]


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else "help"
    if cmd == "login":
        login()
    elif cmd == "whoami":
        print(current_user())
    elif cmd == "training":
        # training [start <gameSlug> [challenge|exploration]]: list or start a session for a past game
        if len(sys.argv) > 2 and sys.argv[2] == "start":
            mode = sys.argv[4] if len(sys.argv) > 4 else "exploration"
            print(hub("POST", f"/api/training/{sys.argv[3]}/start?mode={mode}"))
        else:
            print(json.dumps(hub("GET", "/api/training/active")[1], indent=1))
            for g in hub("GET", "/api/games")[1]:
                print(g["slug"], "|", g["name"], g["baseUrl"], "| levels", g["totalLevels"])
    elif cmd == "contests":
        for c in contests():
            print(c["id"], c["status"], c["startTime"], c["slug"], "|", c["gameName"], c["gameBaseUrl"], "| levels", c["totalLevels"])
    elif cmd == "info":
        print(json.dumps(Game(sys.argv[2]).info(), indent=1))
    elif cmd == "files":
        print(Game(sys.argv[2]).level_files(int(sys.argv[3])))
    elif cmd == "input":
        code, raw = Game(sys.argv[2]).input_file(int(sys.argv[3]), sys.argv[4])
        print(code); sys.stdout.write(raw.decode(errors="replace"))
    elif cmd == "submit":
        print(json.dumps(Game(sys.argv[2]).submit(int(sys.argv[3]), sys.argv[4], sys.argv[5]), indent=1))
    else:
        print(__doc__)


if __name__ == "__main__":
    main()
