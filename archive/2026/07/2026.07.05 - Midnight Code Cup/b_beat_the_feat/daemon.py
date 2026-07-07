#!/usr/bin/env python3
"""Poll the contest API; for each new active record: extract, solve, submit.

Runs forever. State (already submitted records) persists in work/submitted.json.
"""
import json
import os
import subprocess
import sys
import threading
import time
import urllib.error
import urllib.request
import uuid
from datetime import datetime, timezone
from pathlib import Path

TASK_DIR = Path(__file__).resolve().parent
WORK = TASK_DIR / "work"
SOLVER = TASK_DIR / "../../target/release/b"
API = "https://finals.midnightcodecup.org/api/v1"
KEY = (TASK_DIR / "../api-key.txt").read_text().strip()
STATE = WORK / "submitted.json"
POLL_SECONDS = 10


def log(msg: str) -> None:
    line = f"[{datetime.now(timezone.utc).strftime('%H:%M:%S')}] {msg}"
    print(line, flush=True)
    with open(WORK / "daemon.log", "a") as f:
        f.write(line + "\n")


def api_get(path: str):
    req = urllib.request.Request(API + path, headers={"Authorization": f"Bearer {KEY}"})
    with urllib.request.urlopen(req, timeout=20) as resp:
        return json.loads(resp.read())


def api_submit(rec: str, content: bytes):
    boundary = uuid.uuid4().hex
    body = (
        f"--{boundary}\r\n"
        f'Content-Disposition: form-data; name="file"; filename="{rec}.txt"\r\n'
        f"Content-Type: application/octet-stream\r\n\r\n"
    ).encode() + content + f"\r\n--{boundary}--\r\n".encode()
    req = urllib.request.Request(
        f"{API}/problems/B/submit-current-test",
        data=body,
        headers={
            "Authorization": f"Bearer {KEY}",
            "Content-Type": f"multipart/form-data; boundary={boundary}",
        },
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.loads(resp.read())


def load_state() -> dict:
    if STATE.exists():
        return json.loads(STATE.read_text())
    return {}


def save_state(state: dict) -> None:
    STATE.write_text(json.dumps(state, indent=1))


def fetch_wav(rec: str, password: str, url: str) -> None:
    zip_path = TASK_DIR / f"{rec}.zip"
    if zip_path.exists():
        subprocess.run(
            ["7z", "x", "-y", f"-p{password}", f"-o{WORK}", str(zip_path)],
            check=True,
            capture_output=True,
        )
    else:
        log(f"{rec}: no local zip, downloading")
        urllib.request.urlretrieve(url, WORK / f"{rec}.wav")


def trivial_perm(rec: str) -> bytes:
    r = int(rec.split("-")[1])
    n = 3 * 2 ** (13 - r)
    return (" ".join(map(str, range(n))) + "\n").encode()


def write_ref(rec: str, perm_path: Path | None = None) -> None:
    """Store restored audio as a reference for later records of this stage."""
    toks = (WORK / f"{rec}.in").read_text().split()
    n, size = int(toks[0]), int(toks[1])
    a = toks[2:]
    perm_path = perm_path or WORK / f"{rec}.txt"
    perm = [int(t) for t in perm_path.read_text().split()]
    cs = size // n
    out: list[str] = []
    for p in perm:
        out.extend(a[p * cs : (p + 1) * cs])
    (WORK / f"{rec}-rest.txt").write_text(f"{size}\n" + " ".join(out) + "\n")


backfill_lock = threading.Lock()


def backfill_stage(stage: str, upto: int) -> None:
    """Recover any earlier records of this stage we failed to solve (outage,
    crash): fetch past passwords, extract, solve, and write their refs so the
    reference chain stays intact for the remaining records."""
    missing = [
        r for r in range(1, upto) if not (WORK / f"{stage}-{r:02d}-rest.txt").exists()
    ]
    if not missing or not backfill_lock.acquire(blocking=False):
        return
    try:
        past = {t["id"]: t.get("password") for t in api_get("/problems/B/scheduled-groups/past-tests")}
        for r in missing:
            rec = f"{stage}-{r:02d}"
            pw = past.get(rec)
            if not pw:
                continue
            try:
                fetch_wav(rec, pw, "")
                subprocess.run(
                    [sys.executable, str(TASK_DIR / "prep.py"), rec, pw],
                    check=True,
                    capture_output=True,
                )
                env = dict(os.environ)
                refs = [
                    p for p in sorted(WORK.glob(f"{stage}-*-rest.txt")) if p.exists()
                ]
                if refs:
                    env["B_REF"] = ":".join(map(str, refs))
                with open(WORK / f"{rec}.in", "rb") as fi, open(WORK / f"{rec}.bf.txt", "wb") as fo:
                    subprocess.run(
                        ["nice", "-n", "15", str(SOLVER)],
                        stdin=fi,
                        stdout=fo,
                        stderr=subprocess.DEVNULL,
                        check=True,
                        timeout=150,
                        env=env,
                    )
                write_ref(rec, WORK / f"{rec}.bf.txt")
                log(f"backfill {rec}: ref recovered")
            except Exception as e:  # noqa: BLE001
                log(f"backfill {rec}: failed ({e})")
    except Exception as e:  # noqa: BLE001
        log(f"backfill {stage}: {e}")
    finally:
        backfill_lock.release()


refine_lock = threading.Lock()


def refine_stage(stage: str, upto: int) -> None:
    """Re-solve earlier records with the newest references. Their windows are
    closed, but their improved reconstructions sharpen refs for what's next."""
    if not refine_lock.acquire(blocking=False):
        return
    try:
        for r in range(1, upto):
            rec = f"{stage}-{r:02d}"
            inp = WORK / f"{rec}.in"
            refs = [
                p
                for p in sorted(WORK.glob(f"{stage}-*-rest.txt"))
                if p.name != f"{rec}-rest.txt"
            ]
            if not inp.exists() or not refs:
                continue
            env = dict(os.environ)
            env["B_REF"] = ":".join(map(str, refs))
            tmp = WORK / f"{rec}.refine.txt"
            try:
                with open(inp, "rb") as fi, open(tmp, "wb") as fo:
                    subprocess.run(
                        ["nice", "-n", "15", str(SOLVER)],
                        stdin=fi,
                        stdout=fo,
                        stderr=subprocess.DEVNULL,
                        check=True,
                        timeout=150,
                        env=env,
                    )
                write_ref(rec, tmp)
            except Exception as e:  # noqa: BLE001
                log(f"refine {rec}: failed ({e})")
        log(f"refine {stage}: refreshed refs for records < {upto}")
    finally:
        refine_lock.release()


def solve_record(rec: str, password: str, url: str) -> bytes:
    out_path = WORK / f"{rec}.txt"
    if out_path.exists() and out_path.stat().st_size > 0:
        return out_path.read_bytes()
    fetch_wav(rec, password, url)
    subprocess.run(
        [sys.executable, str(TASK_DIR / "prep.py"), rec, password], check=True, capture_output=True
    )
    env = dict(os.environ)
    refs = sorted(WORK.glob(f"{rec[:2]}-*-rest.txt")) + [WORK / f"{rec[:2]}-ref.txt"]
    refs = [p for p in refs if p.exists()]
    if refs:
        env["B_REF"] = ":".join(map(str, refs))
    with open(WORK / f"{rec}.in", "rb") as fin, open(out_path, "wb") as fout:
        subprocess.run(
            [str(SOLVER)],
            stdin=fin,
            stdout=fout,
            stderr=subprocess.DEVNULL,
            check=True,
            timeout=150,
            env=env,
        )
    try:
        write_ref(rec)
    except Exception as e:  # noqa: BLE001 — reference is best-effort
        log(f"{rec}: write_ref failed: {e}")
    return out_path.read_bytes()


def handle(rec: str, password: str, url: str, state: dict) -> None:
    t0 = time.time()
    try:
        ans = solve_record(rec, password, url)
        if not ans.strip():
            raise ValueError("empty solver output")
    except Exception as e:  # noqa: BLE001 — never skip a window
        log(f"{rec}: solve failed ({e}), falling back to trivial permutation")
        ans = trivial_perm(rec)
    try:
        res = api_submit(rec, ans)
    except urllib.error.HTTPError as e:
        body = e.read()[:200]
        log(f"{rec}: submit HTTP {e.code}: {body}")
        if 400 <= e.code < 500:
            state[rec] = {"submission": None, "error": f"{e.code} {body!r}"}
            save_state(state)
        return
    sub_id = res.get("id")
    state[rec] = {"submission": sub_id, "at": datetime.now(timezone.utc).isoformat()}
    save_state(state)
    log(f"{rec}: submitted id={sub_id} in {time.time() - t0:.1f}s")
    # NB: refine_stage disabled — re-solving earlier records homogenized errors
    # across refs, letting false hints through the unique-successor veto
    # (stage 3 records 7-8 scored below stage 2's despite refs).
    threading.Thread(
        target=backfill_stage, args=(rec[:2], int(rec[3:5])), daemon=True
    ).start()
    time.sleep(2)
    try:
        sub = api_get(f"/submissions/{sub_id}")
        for t in sub.get("tests", []):
            log(f"{rec}: status={t.get('status')} {(t.get('log') or '').strip()}")
    except Exception as e:  # noqa: BLE001
        log(f"{rec}: status check failed: {e}")


def main() -> None:
    WORK.mkdir(exist_ok=True)
    state = load_state()
    log(f"daemon started, {len(state)} records already submitted")
    while True:
        try:
            prob = api_get("/problems/B")
            active = (prob.get("submit") or {}).get("active_test")
            if active:
                rec = active["id"]
                if rec not in state:
                    log(f"{rec}: new window, ends {active.get('ends_at')}")
                    handle(rec, active["password"], active.get("download_url", ""), state)
            else:
                log("no active test")
        except Exception as e:  # noqa: BLE001
            log(f"error: {e}")
        time.sleep(POLL_SECONDS)


if __name__ == "__main__":
    main()
