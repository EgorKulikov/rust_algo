"""Every ~3 min: score each out/NN.out; if better than last submitted (by
>0.1%), submit it. State in submitted.json. Skips files mid-write via retry.
"""
import json
import os
import sys
import time

sys.path.insert(0, ".")
import mcc_a

STATE = "submitted.json"


def main():
    state = json.load(open(STATE)) if os.path.exists(STATE) else {}
    while True:
        for dd in range(1, 13):
            key = f"{dd:02d}"
            err = None
            snap = f"/tmp/snap_{key}.out"
            for suffix in ("", "g", "w"):
                path = os.path.join(mcc_a.OUT, f"{key}{suffix}.out")
                if not os.path.exists(path):
                    continue
                try:
                    with open(path, "rb") as f:
                        data = f.read()
                    cand = f"/tmp/cand_{key}.out"
                    with open(cand, "wb") as f:
                        f.write(data)
                    e = mcc_a.score_file(dd, cand)
                except Exception as ex:
                    print(f"{key}{suffix}: score failed ({ex}), skip", flush=True)
                    continue
                if err is None or e < err:
                    err = e
                    os.replace(cand, snap)
            if err is None:
                continue
            prev = state.get(key)
            if prev is None or err < prev * 0.999:
                try:
                    sub = os.path.join("/tmp", f"{key}.out")
                    os.replace(snap, sub)
                    resp = mcc_a.submit([sub])
                    state[key] = err
                    json.dump(state, open(STATE, "w"), indent=1)
                    print(f"{key}: submitted err={err:,} (id {resp.get('id')})", flush=True)
                    time.sleep(60)
                except Exception as e:
                    print(f"{key}: submit failed ({e})", flush=True)
        time.sleep(180)


if __name__ == "__main__":
    main()
