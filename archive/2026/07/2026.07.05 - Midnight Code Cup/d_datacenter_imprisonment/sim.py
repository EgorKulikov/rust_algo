#!/usr/bin/env python3
"""Oracle simulator for MCC D batch band: zero-latency k-stream exploration.

Loads a maze from a merlin event log and simulates bidirectional frontier
exploration with k parallel queries per turn and instant shared results —
the architecture upper bound for coordinator+workers at K=k.
"""
import json, sys, heapq


def load(path):
    ev = json.load(open(path))
    n = ev["n"]
    m = (n + 1) // 2
    grid = ev["maze"]
    return n, m, grid


class Sim:
    def __init__(self, m, grid):
        self.m = m
        self.grid = grid
        self.HM = m * (m - 1)
        self.E = 2 * self.HM

    def einfo(self, e):
        m = self.m
        if e < self.HM:
            i, j = divmod(e, m - 1)
            return i * m + j, i * m + j + 1, 2 * i + 1, 2 * j + 2
        x = e - self.HM
        i, j = divmod(x, m)
        return i * m + j, (i + 1) * m + j, 2 * i + 2, 2 * j + 1

    def eid(self, i, j, d):
        m = self.m
        DI = (-1, 0, 1, 0)
        DJ = (0, -1, 0, 1)
        ni, nj = i + DI[d], j + DJ[d]
        if not (0 <= ni < m and 0 <= nj < m):
            return -1
        if d == 3:
            return i * (m - 1) + j
        if d == 1:
            return i * (m - 1) + (j - 1)
        if d == 2:
            return self.HM + i * m + j
        return self.HM + (i - 1) * m + j

    def is_open(self, e):
        _, _, r, c = self.einfo(e)
        return self.grid[r - 1][c - 1] == "."

    def run(self, k, C=3, lm_stride=32, deduce=True, alpha=0.0, single=False):
        m = self.m
        E = self.E
        start, goal = 0, m * m - 1
        side = [-1] * (m * m)
        side[start], side[goal] = 0, 1
        blobs = [[start], [goal]]
        depth = [0] * (m * m)
        est = bytearray(E)  # 0 unknown, 2 known
        pq = []
        seq = 0
        unk = [0] * (m * m)
        opn = [0] * (m * m)
        for r in range(m * m):
            i, j = divmod(r, m)
            for d in range(4):
                if self.eid(i, j, d) >= 0:
                    unk[r] += 1

        def dfar(s_other, i, j):
            blob = blobs[s_other]
            stride = max(1, len(blob) // lm_stride)
            best = 1 << 60
            for t in range(0, len(blob), stride):
                rr = blob[t]
                dd = abs(rr // m - i) + abs(rr % m - j)
                if dd < best:
                    best = dd
            return best

        bridges = []

        def push_room(room):
            nonlocal seq
            s = side[room]
            i, j = divmod(room, m)
            for d in range(4):
                e = self.eid(i, j, d)
                if e < 0 or est[e]:
                    continue
                u, v, _, _ = self.einfo(e)
                w = v if u == room else u
                if side[w] == s:
                    mark(e, False, free=True)
                    continue
                if side[w] != -1:
                    bridges.append(e)
                    continue
                wi, wj = divmod(w, m)
                seq += 1
                heapq.heappush(pq, (dfar(1 - s, wi, wj) + alpha * (depth[room] + 1), seq, e))

        self.done = False
        Q = 0

        def mark(e, open_, free=False):
            nonlocal Q
            if est[e]:
                return
            est[e] = 2
            u, v, _, _ = self.einfo(e)
            unk[u] -= 1
            unk[v] -= 1
            if open_:
                opn[u] += 1
                opn[v] += 1
                su, sv = side[u], side[v]
                if su != -1 and sv != -1:
                    if su != sv:
                        self.done = True
                    return
                if su == -1 and sv == -1:
                    return  # floating open: recorded, no expansion in 2-side sim
                if su == -1:
                    u, v = v, u
                    su = sv
                side[v] = su
                depth[v] = depth[u] + 1
                blobs[su].append(v)
                push_room(v)
            if deduce:
                for r in (u, v):
                    if opn[r] == 0 and unk[r] == 1:
                        i, j = divmod(r, m)
                        for d in range(4):
                            ee = self.eid(i, j, d)
                            if ee >= 0 and not est[ee]:
                                mark(ee, True, free=True)
                                break

        push_room(start)
        if not single:
            push_room(goal)
        turns = 0
        while not self.done:
            turns += 1
            got = 0
            while got < k and not self.done:
                e = -1
                while bridges:
                    cand = bridges.pop()
                    if not est[cand]:
                        u, v, _, _ = self.einfo(cand)
                        if side[u] != -1 and side[u] == side[v]:
                            mark(cand, False, free=True)
                            continue
                        e = cand
                        break
                if e < 0:
                    while pq:
                        _, _, cand = heapq.heappop(pq)
                        if est[cand]:
                            continue
                        u, v, _, _ = self.einfo(cand)
                        if side[u] != -1 and side[u] == side[v]:
                            mark(cand, False, free=True)
                            continue
                        e = cand
                        break
                if e < 0:
                    if got == 0:
                        raise RuntimeError("frontier exhausted before claim")
                    break
                Q += 1
                got += 1
                mark(e, self.is_open(e))
        return Q, turns



    def run_batched(self, k, B, C=3, lm_stride=32):
        """1 fresh stream + (k-1) workers executing B-sized batches assigned
        from the dial at refill time — isolates assignment staleness."""
        m = self.m
        start, goal = 0, m * m - 1
        side = [-1] * (m * m)
        side[start], side[goal] = 0, 1
        blobs = [[start], [goal]]
        est = bytearray(self.E)
        pq = []
        seq = 0
        unk = [0] * (m * m)
        opn = [0] * (m * m)
        for r in range(m * m):
            i, j = divmod(r, m)
            for d in range(4):
                if self.eid(i, j, d) >= 0:
                    unk[r] += 1

        def dfar(s_other, i, j):
            blob = blobs[s_other]
            stride = max(1, len(blob) // lm_stride)
            best = 1 << 60
            for t in range(0, len(blob), stride):
                rr = blob[t]
                dd = abs(rr // m - i) + abs(rr % m - j)
                if dd < best:
                    best = dd
            return best

        bridges = []

        def push_room(room):
            nonlocal seq
            s = side[room]
            i, j = divmod(room, m)
            for d in range(4):
                e = self.eid(i, j, d)
                if e < 0 or est[e]:
                    continue
                u, v, _, _ = self.einfo(e)
                w = v if u == room else u
                if side[w] == s:
                    mark(e, False)
                    continue
                if side[w] != -1:
                    bridges.append(e)
                    continue
                wi, wj = divmod(w, m)
                seq += 1
                heapq.heappush(pq, (dfar(1 - s, wi, wj), seq, e))

        self.done = False

        def mark(e, open_):
            if est[e]:
                return
            est[e] = 2
            u, v, _, _ = self.einfo(e)
            unk[u] -= 1
            unk[v] -= 1
            if open_:
                opn[u] += 1
                opn[v] += 1
                su, sv = side[u], side[v]
                if su != -1 and sv != -1:
                    if su != sv:
                        self.done = True
                    return
                if su == -1 and sv == -1:
                    return
                if su == -1:
                    u, v = v, u
                    su = sv
                side[v] = su
                blobs[su].append(v)
                push_room(v)
            for r in (u, v):
                if opn[r] == 0 and unk[r] == 1:
                    i, j = divmod(r, m)
                    for d in range(4):
                        ee = self.eid(i, j, d)
                        if ee >= 0 and not est[ee]:
                            mark(ee, True)
                            break

        def pop_valid():
            while bridges:
                cand = bridges.pop()
                if est[cand]:
                    continue
                u, v, _, _ = self.einfo(cand)
                if side[u] != -1 and side[u] == side[v]:
                    mark(cand, False)
                    continue
                return cand
            while pq:
                _, _, cand = heapq.heappop(pq)
                if est[cand]:
                    continue
                u, v, _, _ = self.einfo(cand)
                if side[u] != -1 and side[u] == side[v]:
                    mark(cand, False)
                    continue
                return cand
            return -1

        push_room(start)
        push_room(goal)
        Q = 0
        turns = 0
        wq = [[] for _ in range(k - 1)]
        while not self.done:
            turns += 1
            # coordinator: one fresh pop
            e = pop_valid()
            if e >= 0:
                Q += 1
                mark(e, self.is_open(e))
            if self.done:
                break
            # workers: one edge each from their (possibly stale) batch
            for w in range(k - 1):
                while wq[w]:
                    e = wq[w].pop()
                    if est[e]:
                        continue
                    Q += 1
                    mark(e, self.is_open(e))
                    break
                else:
                    # refill from current dial
                    if getattr(self, "spread", 0):
                        cands = []
                        for _ in range(B * 3):
                            e = pop_valid()
                            if e < 0:
                                break
                            cands.append(e)
                        batch = []
                        if cands:
                            pts = []
                            for e in cands:
                                u, v, _, _ = self.einfo(e)
                                pts.append((u // m, u % m))
                            chosen = [0]
                            while len(chosen) < min(B, len(cands)):
                                bi, bd = -1, -1
                                for t in range(len(cands)):
                                    if t in chosen:
                                        continue
                                    dd = min(abs(pts[t][0] - pts[c][0]) + abs(pts[t][1] - pts[c][1]) for c in chosen)
                                    if dd > bd:
                                        bd, bi = dd, t
                                chosen.append(bi)
                            cset = set(chosen)
                            batch = [cands[t] for t in sorted(cset)]
                            # unused candidates back to the dial at original-ish price
                            for t in range(len(cands)):
                                if t not in cset:
                                    e = cands[t]
                                    u, v, _, _ = self.einfo(e)
                                    wroom = v if side[u] != -1 else u
                                    wi, wj = divmod(wroom, m)
                                    nonlocal_seq = len(cands) + t
                                    heapq.heappush(pq, (dfar(1 - max(side[u], side[v], 0), wi, wj), 10**9 + nonlocal_seq, e))
                    else:
                        batch = []
                        for _ in range(B):
                            e = pop_valid()
                            if e < 0:
                                break
                            batch.append(e)
                    wq[w] = batch[::-1]
                    if wq[w]:
                        e = wq[w].pop()
                        Q += 1
                        mark(e, self.is_open(e))
                if self.done:
                    break
        return Q, turns


if __name__ == "__main__":
    import glob

    SP = "/tmp/claude-1000/-home-egor-proj-rust-algo/eef97540-2ee7-4352-9aca-ec12b3ebbad2/scratchpad"
    for path in sorted(glob.glob(f"{SP}/maze501_*.json"))[: int(sys.argv[1]) if len(sys.argv) > 1 else 5]:
        n, m, grid = load(path)
        sim = Sim(m, grid)
        row = [path.split("_")[-1][0]]
        for k in (1, 3, 6):
            Q, T = sim.run(k)
            row.append(f"k={k}: Q={Q} T={T}")
        print("seed", *row, flush=True)
