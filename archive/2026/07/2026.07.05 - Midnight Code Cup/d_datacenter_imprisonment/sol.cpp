// MCC D "Datacenter Imprisonment" — parallel speculative bidirectional exploration.
//
// Maze facts (from merlin-source): N odd, rooms at (odd,odd) always empty,
// (even,even) always wall; empty cells form a spanning tree of all rooms.
// => only the 2m(m-1) passage cells are unknown (m=(N+1)/2), and a passage
// between two rooms already connected through revealed passages must be a wall.
//
// Roles: agent 0 coordinates, agents 1..K-1 are query workers.
// Any cell may be probed regardless of connectivity, so exploration is NOT
// latency-bound: the coordinator prices every unknown passage in a bucket dial
// with p = C*ringDist(nearest blob) + estDist(other blob) — with few agents the
// dial serves pure frontier (ring 0), with many agents it reaches into
// speculative rings so all workers stay busy. Revealed opens feed a global DSU
// (floating components merge when touched); claim fires the moment start and
// goal connect. Direct S<->G bridge passages are queried by the coordinator
// itself for 1-turn latency.
#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

static char ibuf[1 << 16];
static string rd() {
    if (!fgets(ibuf, sizeof ibuf, stdin)) exit(0);
    string s(ibuf);
    while (!s.empty() && (s.back() == '\n' || s.back() == '\r')) s.pop_back();
    return s;
}
static void wr(const string& s) {
    fputs(s.c_str(), stdout);
    fputc('\n', stdout);
    fflush(stdout);
}

static ll N, K, ID, MSGLEN;
static ll m, HM, E;

static ll envll(const char* name, ll dflt) {
    const char* v = getenv(name);
    return v ? atoll(v) : dflt;
}
// shared tunables (same env in every agent)
static ll pR;         // worker report threshold
static ll pTargetOut; // max in-flight edges per worker
static ll pC;         // ring-distance weight in dial priority
static ll pKR;        // speculative ring depth
static bool pStatic;  // static full-scan mode (large K)
static ll pP;         // static-mode report period
static ll pNE;        // max edges per assignment message
static ll pPRB;       // dedicated chain-probe workers (static band)
static ll pTSH;       // 1 = diagonal-rotated (diamond) tiles
static ll pTS;        // lease tile size (rooms per side)
static ll pMODE;      // adaptive dispatch: 0 = edge batches, 1 = tile leases
static void initParams() {
    ll W = max(K - 1, 1LL);
    pNE = min({(MSGLEN - 1) / 3, 84LL, envll("D_NE", 84)});
    ll nE = pNE;
    ll Weff = W;
    if (envll("D_DUAL", 0) && W >= envll("D_DUALLO", 6) && W <= envll("D_DUALHI", 12))
        Weff = max(1LL, (K - 2) / 2);
    pR = max(envll("D_RMIN", 16), min({160LL, envll("D_RMUL", 2) * Weff, (MSGLEN - 6) * 6}));
    pTargetOut = min(2 * nE, max(2 * pR, envll("D_TMIN", 24)));
    pC = envll("D_C", 3);
    pStatic = W >= envll("D_WTH", 22) || (W >= 13 && E / W <= envll("D_SLEASE", 3000)) || (W >= 9 && E / W <= envll("D_SRTH", 400));
    if (envll("D_MODE", 2) == 3) pStatic = false;  // explicit heads override
    pP = min(max(envll("D_P", 56), (10 * W + 7) / 8), (MSGLEN - 6) * 6);
    pTS = max(4LL, envll("D_TS", 10));
    pTSH = envll("D_TSH", 0);
    pPRB = (W >= envll("D_PRBW", 9999)) ? envll("D_PRB", 2) : 0;
    pMODE = envll("D_MODE", 2);
    if (pMODE == 2) {  // auto: tile leases pay at high worker counts & tiny mazes
        pMODE = (W >= envll("D_LW", 13) || (m <= envll("D_LM", 60) && W <= envll("D_LMW", 5))) ? 1 : 0;
        if (pMODE == 0 && envll("D_DUAL", 0) && W >= envll("D_DUALLO", 6) && W <= envll("D_DUALHI", 12))
            pMODE = 5;
    }
    // ring depth: 8 across adaptive bands, except big-maze lease prefers 4
    pKR = envll("D_KR", pMODE == 1 && (m > envll("D_KRM", 130) || m <= envll("D_LM", 60)) ? 4 : 8);
    if (envll("D_SYNC", 0) && W >= envll("D_SYNCLO", 21)) pMODE = 6;
    if (envll("D_STRM", 0) && W >= envll("D_STRMLO", 21)) pMODE = 7;
}
static ll G;  // tiles per side
static inline ll tileOfRoom(ll room) {
    ll i = room / m, j = room % m;
    if (pTSH) {
        ll a = i + j, b = i - j + (m - 1);
        return (a / pTS) * G + (b / pTS);
    }
    return (i / pTS) * G + (j / pTS);
}

const int DI[4] = {-1, 0, 1, 0}, DJ[4] = {0, -1, 0, 1};
const char DCH[4] = {'U', 'L', 'D', 'R'};

struct EI {
    int u, v;
    ll r, c;
};
static EI edgeInfo(ll e) {
    if (e < HM) {
        ll i = e / (m - 1), j = e % (m - 1);
        return {int(i * m + j), int(i * m + j + 1), 2 * i + 1, 2 * j + 2};
    }
    ll x = e - HM, i = x / m, j = x % m;
    return {int(i * m + j), int((i + 1) * m + j), 2 * i + 2, 2 * j + 1};
}
static ll edgeId(ll i, ll j, int d) {
    ll ni = i + DI[d], nj = j + DJ[d];
    if (ni < 0 || nj < 0 || ni >= m || nj >= m) return -1;
    if (d == 3) return i * (m - 1) + j;
    if (d == 1) return i * (m - 1) + (j - 1);
    if (d == 2) return HM + i * m + j;
    return HM + (i - 1) * m + j;
}

static string enc(ll x, int len) {
    string s(len, '!');
    for (int p = len - 1; p >= 0; p--) {
        s[p] = char(33 + x % 94);
        x /= 94;
    }
    return s;
}
static ll dec(const char* p, int len) {
    ll x = 0;
    for (int i = 0; i < len; i++) x = x * 94 + (p[i] - 33);
    return x;
}

// ---------------- worker ----------------
static void worker() {
    deque<ll> wq;
    string bits;
    ll reported = 0;
    const ll R = pR;
    const ll nE = pNE;
    int pollTimer = 0;
    const string boss = to_string(pMODE == 5 ? (ID % 2) : 0);
    // local map from own results: provable walls / forced opens answered free
    vector<int> du(m * m);
    iota(du.begin(), du.end(), 0);
    auto find2 = [&](int x) {
        while (du[x] != x) x = du[x] = du[du[x]];
        return x;
    };
    const bool dedOn = K - 1 >= envll("D_DEDW", 6);
    vector<signed char> lu(m * m, 0), lo(m * m, 0);
    vector<signed char> res(E, 2);  // my answer per edge; 2 = unknown
    for (ll r = 0; r < m * m; r++)
        for (int d = 0; d < 4; d++)
            if (edgeId(r / m, r % m, d) >= 0) lu[r]++;
    auto localMark = [&](ll e, bool open) {
        res[e] = open;
        EI x = edgeInfo(e);
        lu[x.u]--;
        lu[x.v]--;
        if (open) {
            lo[x.u]++;
            lo[x.v]++;
            du[find2(x.u)] = find2(x.v);
        }
    };
    auto poll = [&]() {
        wr("< " + boss);
        string rep = rd();
        pollTimer = 0;
        if (rep[0] != '-') {
            size_t sp = rep.find(' ');
            const char* b = rep.c_str() + sp + 1;
            size_t blen = rep.size() - sp - 1;
            for (size_t p = 1; p + 3 <= blen; p += 3) wq.push_back(dec(b + p, 3));
        }
    };
    while (true) {
        if ((ll)bits.size() >= R || (wq.empty() && !bits.empty())) {
            string body = "R";
            body += enc(reported, 3);
            body += enc((ll)bits.size(), 2);
            for (size_t i = 0; i < bits.size(); i += 6) {
                int v = 0;
                for (size_t k = i; k < min(bits.size(), i + 6); k++)
                    v |= (bits[k] - '0') << (5 - (k - i));
                body += char(33 + v);
            }
            wr("> " + boss + " " + body);
            reported += (ll)bits.size();
            bits.clear();
            rd();
        } else if (wq.empty()) {
            poll();
        } else if (++pollTimer >= 12 && (ll)wq.size() < 2 * nE) {
            poll();
        } else {
            ll e = wq.front();
            wq.pop_front();
            EI x = edgeInfo(e);
            if (res[e] != 2) {  // re-assigned duplicate: answer from memory
                bits += char('0' + res[e]);
                continue;
            }
            if (dedOn && find2(x.u) == find2(x.v)) {  // provable wall: free
                bits += '0';
                localMark(e, false);
                continue;
            }
            if (dedOn && ((lo[x.u] == 0 && lu[x.u] == 1) ||
                          (lo[x.v] == 0 && lu[x.v] == 1))) {  // forced open
                bits += '1';
                localMark(e, true);
                continue;
            }
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            bool open = rd()[0] == '1';
            bits += open ? '1' : '0';
            localMark(e, open);
        }
    }
}

// ---------------- tile-lease worker (adaptive density inside a tile) ----------------
// Lease msg: 'L' tile(2ch) side(1ch: 'S' targets goal, 'G' targets start,
//            'T' + room(3ch) custom target) seeds(3ch each).
// tile == UNB (94^2-1) => unbounded roaming (island head, small-K mode);
// no seeds + bounded tile => speculative scan of the tile's owned edges;
// no seeds + unbounded => rebuild frontier from own map (retarget).
// Report msg: 'D' [(edge*2+bit) 3ch]* ['E' if lease exhausted].
static const ll UNB = 94 * 94 - 1;
static void workerLease() {
    vector<char> vis(m * m, 0);  // rooms expanded locally (ever)
    vector<char> lkn(E, 0);      // edges I have queried (ever)
    vector<int> visList;
    vector<int> du(m * m);  // local DSU over own opens: same-comp => provable wall
    iota(du.begin(), du.end(), 0);
    auto find2 = [&](int x) {
        while (du[x] != x) x = du[x] = du[du[x]];
        return x;
    };
    string results;
    ll myTile = -1, pend = 0, roamPoll = 0;
    ll tgtI = 0, tgtJ = 0;
    ll qSince = 0;
    const ll brCap = envll("D_BRC", 400);
    priority_queue<pair<ll, ll>, vector<pair<ll, ll>>, greater<>> fr;
    auto pri = [&](ll e) {
        EI x = edgeInfo(e);
        ll a = llabs(x.u / m - tgtI) + llabs(x.u % m - tgtJ);
        ll b = llabs(x.v / m - tgtI) + llabs(x.v % m - tgtJ);
        return min(a, b);
    };
    auto addRoom = [&](ll room) {
        if (vis[room]) return;
        vis[room] = 1;
        visList.push_back((int)room);
        ll i = room / m, j = room % m;
        for (int d = 0; d < 4; d++) {
            ll e = edgeId(i, j, d);
            if (e >= 0 && !lkn[e]) fr.push({pri(e), e});
        }
    };
    auto applyKnown = [&](const char* b, size_t blen) {
        for (size_t p = 1; p + 3 <= blen; p += 3) {
            ll v = dec(b + p, 3);
            ll e = v >> 1;
            if (lkn[e]) continue;
            lkn[e] = 1;
            if (v & 1) {
                EI x = edgeInfo(e);
                du[find2(x.u)] = find2(x.v);
            }
        }
    };
    auto readLease = [&](const char* b, size_t blen) {
        myTile = dec(b + 1, 2);
        qSince = 0;
        size_t p = 4;
        if (b[3] == 'S') {
            tgtI = m - 1;
            tgtJ = m - 1;
        } else if (b[3] == 'G') {
            tgtI = 0;
            tgtJ = 0;
        } else {  // 'T' + room(3ch)
            ll t = dec(b + 4, 3);
            tgtI = t / m;
            tgtJ = t % m;
            p = 7;
        }
        fr = {};
        for (; p + 3 <= blen; p += 3) {
            ll e = dec(b + p, 3);
            if (!lkn[e]) fr.push({pri(e), e});
        }
        if (fr.empty()) {
            if (myTile == UNB) {
                // retarget: rebuild frontier from own map with new priorities
                for (int room : visList) {
                    ll i = room / m, j = room % m;
                    for (int d = 0; d < 4; d++) {
                        ll e = edgeId(i, j, d);
                        if (e >= 0 && !lkn[e]) fr.push({pri(e), e});
                    }
                }
            } else if (pTSH) {
                for (ll r2 = 0; r2 < m * m; r2++)
                    if (tileOfRoom(r2) == myTile)
                        for (int d = 2; d < 4; d++) {
                            ll e = edgeId(r2 / m, r2 % m, d);
                            if (e >= 0 && !lkn[e]) fr.push({pri(e), e});
                        }
            } else {
                // scan lease: all owned unknown edges of the tile
                ll ti = myTile / G, tj = myTile % G;
                for (ll i = ti * pTS; i < min(m, (ti + 1) * pTS); i++)
                    for (ll j = tj * pTS; j < min(m, (tj + 1) * pTS); j++)
                        for (int d = 2; d < 4; d++) {  // D/R edges have u=(i,j)
                            ll e = edgeId(i, j, d);
                            if (e >= 0 && !lkn[e]) fr.push({pri(e), e});
                        }
            }
        }
    };
    while (true) {
        if (myTile < 0) {
            wr("< 0");
            string rep = rd();
            if (rep[0] == '-') continue;
            size_t sp = rep.find(' ');
            const char* b = rep.c_str() + sp + 1;
            if (b[0] == 'L')
                readLease(b, rep.size() - sp - 1);
            else if (b[0] == 'K')
                applyKnown(b, rep.size() - sp - 1);
            continue;
        }
        if (myTile == UNB && ++roamPoll >= 24) {
            roamPoll = 0;
            wr("< 0");
            string rep = rd();
            if (rep[0] != '-') {
                size_t sp = rep.find(' ');
                const char* b = rep.c_str() + sp + 1;
                if (b[0] == 'L')
                    readLease(b, rep.size() - sp - 1);
                else if (b[0] == 'K')
                    applyKnown(b, rep.size() - sp - 1);
                roamPoll = 20;  // messages tend to arrive in runs: poll again soon
            }
            continue;
        }
        if (!fr.empty() && pend < 40) {
            auto [p, e] = fr.top();
            fr.pop();
            if (lkn[e]) continue;
            EI x = edgeInfo(e);
            if (find2(x.u) == find2(x.v)) {  // provable wall: free skip
                lkn[e] = 1;
                continue;
            }
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            bool open = rd()[0] == '1';
            lkn[e] = 1;
            qSince++;
            results += enc(e * 2 + open, 3);
            pend++;
            if (open) {
                du[find2(x.u)] = find2(x.v);
                if (myTile == UNB || tileOfRoom(x.u) == myTile) addRoom(x.u);
                if (myTile == UNB || tileOfRoom(x.v) == myTile) addRoom(x.v);
            }
        } else {
            bool done = fr.empty() || (myTile == UNB && qSince >= brCap);
            string body = "D" + results;
            if (done) body += 'E';
            wr("> 0 " + body);
            results.clear();
            pend = 0;
            rd();
            if (done) {
                myTile = -1;
                fr = {};  // capped branch: dial re-owns the abandoned frontier
            }
        }
    }
}

// ---------------- regional autonomous scan (large K, D_RG=1) ----------------
// Worker ID owns the contiguous tile range [ (ID-1)*G²/W, ID*G²/W ): it resolves
// every owned edge, but adaptively — opens first, and any edge whose endpoints
// its local DSU already connects is a provable wall (skipped, never queried).
// Degree rule: a room with no known open and one unknown edge left must open
// there (the tree spans all rooms) — deduced free, reported like a query.
// Reports 'D' (edge*2+bit)*3ch, then 'E' + halt when the region is resolved.
static void workerRegional() {
    ll W = K - 1;
    // squarish regions: G tile-rows split into gr bands; band i split into
    // ((i+1)W/gr - iW/gr) column chunks; region ids 0..W-1 cover all tiles
    ll gr = max(1LL, (ll)llround(sqrt((double)W)));
    // room-space partition: equal-area squarish regions (±1 row/col)
    auto regionOf = [&](ll room) {
        ll ri = room / m, rj = room % m;
        ll i = min(gr - 1, ri * gr / m);
        while (i > 0 && ri < i * m / gr) i--;
        while (i + 1 < gr && ri >= (i + 1) * m / gr) i++;
        ll w0 = i * W / gr, wi = max(1LL, (i + 1) * W / gr - w0);
        ll k = min(wi - 1, rj * wi / m);
        while (k > 0 && rj < k * m / wi) k--;
        while (k + 1 < wi && rj >= (k + 1) * m / wi) k++;
        return w0 + k;
    };
    vector<char> lkn(E, 0);
    vector<int> du(m * m);
    iota(du.begin(), du.end(), 0);
    auto find2 = [&](int x) {
        while (du[x] != x) x = du[x] = du[du[x]];
        return x;
    };
    vector<signed char> unkCnt(m * m, 0), opnCnt(m * m, 0);
    for (ll r = 0; r < m * m; r++) {
        ll i = r / m, j = r % m;
        for (int d = 0; d < 4; d++)
            if (edgeId(i, j, d) >= 0) unkCnt[r]++;
    }
    auto bandKey = [&](ll e) {
        EI x = edgeInfo(e);
        ll iu = x.u / m, ju = x.u % m, iv = x.v / m, jv = x.v % m;
        if (envll("D_ORD", 0))
            return min(iu + ju, iv + jv) +
                   min(2 * (m - 1) - iu - ju, 2 * (m - 1) - iv - jv);
        return llabs(iu - ju) + llabs(iv - jv);
    };
    priority_queue<pair<ll, ll>, vector<pair<ll, ll>>, greater<>> fr;
    const bool ctr = envll("D_CTR", 0);
    ll ci = 0, cj = 0, nOwn = 0;
    for (ll e = 0; e < E; e++)
        if (regionOf(edgeInfo(e).u) == ID - 1) {
            EI x = edgeInfo(e);
            ci += x.u / m;
            cj += x.u % m;
            nOwn++;
        }
    if (nOwn) {
        ci /= nOwn;
        cj /= nOwn;
    }
    auto centerKey = [&](ll e) {
        EI x = edgeInfo(e);
        return llabs(x.u / m - ci) + llabs(x.u % m - cj);
    };
    if (ctr && nOwn) {
        // island mode: grow a local blob from the region center; the rest of
        // the region is coverage-fallback via a second phase
        ll seed = ci * m + cj;
        ll i = seed / m, j = seed % m;
        for (int d = 0; d < 4; d++) {
            ll e = edgeId(i, j, d);
            if (e >= 0 && regionOf(edgeInfo(e).u) == ID - 1) fr.push({0, e});
        }
    } else {
        for (ll e = 0; e < E; e++)
            if (regionOf(edgeInfo(e).u) == ID - 1) fr.push({bandKey(e), e});
    }
    bool phase2 = !ctr;
    string results;
    ll pend = 0;
    deque<ll> deduce;  // rooms to re-check for the degree rule
    // markKnown: record an edge fact locally; open facts expand + may cascade
    auto markKnown = [&](ll e, bool open) {
        lkn[e] = 1;
        EI x = edgeInfo(e);
        unkCnt[x.u]--;
        unkCnt[x.v]--;
        if (open) {
            opnCnt[x.u]++;
            opnCnt[x.v]++;
            du[find2(x.u)] = find2(x.v);
            for (int rr : {x.u, x.v}) {
                ll i = rr / m, j = rr % m;
                for (int d = 0; d < 4; d++) {
                    ll ee = edgeId(i, j, d);
                    if (ee < 0 || lkn[ee]) continue;
                    if (regionOf(edgeInfo(ee).u) == ID - 1)
                        fr.push({(ctr ? centerKey(ee) : bandKey(ee)) - 100000, ee});
                }
            }
        }
        deduce.push_back(x.u);
        deduce.push_back(x.v);
    };
    auto runDeduce = [&]() {
        while (!deduce.empty() && pend < 80) {
            int r = deduce.front();
            deduce.pop_front();
            if (opnCnt[r] > 0 || unkCnt[r] != 1) continue;
            ll i = r / m, j = r % m;
            for (int d = 0; d < 4; d++) {
                ll e = edgeId(i, j, d);
                if (e < 0 || lkn[e]) continue;
                markKnown(e, true);  // forced open; report it as a fact
                results += enc(e * 2 + 1, 3);
                pend++;
                break;
            }
        }
    };
    while (true) {
        runDeduce();
        if (!fr.empty() && pend < 60) {
            auto [p, e] = fr.top();
            fr.pop();
            if (lkn[e]) continue;
            EI x = edgeInfo(e);
            if (find2(x.u) == find2(x.v)) {  // provable wall, free
                markKnown(e, false);
                continue;
            }
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            bool open = rd()[0] == '1';
            markKnown(e, open);
            results += enc(e * 2 + open, 3);
            pend++;
        } else {
            bool done = fr.empty();
            if (done && !phase2) {
                // island blob exhausted: coverage fallback over the remainder
                phase2 = true;
                for (ll e = 0; e < E; e++)
                    if (!lkn[e] && regionOf(edgeInfo(e).u) == ID - 1)
                        fr.push({centerKey(e), e});
                done = fr.empty();
            }
            string body = "D" + results;
            if (done) body += 'E';
            wr("> 0 " + body);
            results.clear();
            pend = 0;
            rd();
            if (done) break;  // region resolved: switch to work-stealing
        }
    }
    // work-stealing phase: execute 'A' edge batches for straggler regions
    while (true) {
        wr("< 0");
        string rep = rd();
        if (rep[0] == '-') continue;
        size_t sp = rep.find(' ');
        const char* b = rep.c_str() + sp + 1;
        size_t blen = rep.size() - sp - 1;
        if (b[0] != 'A') continue;
        for (size_t p = 1; p + 3 <= blen; p += 3) {
            ll e = dec(b + p, 3);
            if (lkn[e]) continue;
            EI x = edgeInfo(e);
            if (find2(x.u) == find2(x.v)) {
                markKnown(e, false);
                continue;
            }
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            bool open = rd()[0] == '1';
            markKnown(e, open);
            results += enc(e * 2 + open, 3);
            pend++;
            if (pend >= 60) {
                wr("> 0 D" + results);
                results.clear();
                pend = 0;
                rd();
            }
        }
        runDeduce();
        wr("> 0 D" + results + "E");  // idle again
        results.clear();
        pend = 0;
        rd();
    }
}

// prober: executes 'A' chain batches from the coordinator, reports 'D'+'E'
static void proberLoop() {
    while (true) {
        wr("< 0");
        string rep = rd();
        if (rep[0] == '-') continue;
        size_t sp = rep.find(' ');
        const char* b = rep.c_str() + sp + 1;
        size_t blen = rep.size() - sp - 1;
        if (b[0] != 'A') continue;
        string results;
        for (size_t p = 1; p + 3 <= blen; p += 3) {
            ll e = dec(b + p, 3);
            EI x = edgeInfo(e);
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            bool open = rd()[0] == '1';
            results += enc(e * 2 + open, 3);
        }
        wr("> 0 D" + results + "E");
        rd();
    }
}

// ---------------- static full-scan mode (large K) ----------------
// Global band order: edges sorted by (L1 to start corner + L1 to goal corner),
// identical in every agent. Worker j scans order[j-1], order[j-1+W], ... and
// streams packed bitmap reports; the coordinator only ingests and claims.
static vector<int> staticOrder() {
    vector<int> key(E);
    int maxK = 0;
    static const ll oldOrd = envll("D_ORD", 0);
    for (ll e = 0; e < E; e++) {
        EI x = edgeInfo(e);
        ll iu = x.u / m, ju = x.u % m, iv = x.v / m, jv = x.v % m;
        if (oldOrd) {
            ll ds = min(iu + ju, iv + jv);
            ll dg = min((m - 1 - iu) + (m - 1 - ju), (m - 1 - iv) + (m - 1 - jv));
            key[e] = int(ds + dg);
        } else {
            // diagonal-strip order: start-goal paths hug the main diagonal, so
            // scans connect at ~0.68 coverage instead of ~1.0 (measured)
            key[e] = int(llabs(iu - ju) + llabs(iv - jv));
        }
        maxK = max(maxK, key[e]);
    }
    vector<int> cnt(maxK + 2, 0);
    for (ll e = 0; e < E; e++) cnt[key[e] + 1]++;
    for (int k = 1; k <= maxK + 1; k++) cnt[k] += cnt[k - 1];
    vector<int> ord(E);
    for (ll e = 0; e < E; e++) ord[cnt[key[e]]++] = (int)e;
    return ord;
}

// contiguous chunk boundaries with arithmetic stagger: chunk j ~ A + 2(j-1),
// scaled to sum to E — early workers finish first so reports stream into the
// coordinator at its 1-msg/turn intake instead of arriving in one burst
// relay aggregation (drain-bound small mazes): sqrt(W) groups; members send
// chunk bitmaps to their group leader, leaders forward merged 'B' messages to
// the coordinator — parallel ingestion cuts drain from ~W to ~2*sqrt(W) turns.
static ll relS = 0, relG = 0;  // group size / group count (0 = relay off)
static void initRelay() {
    ll W = K - 1;
    if (!pStatic || W < 9 || E / W > envll("D_RELTH", 200)) return;
    ll sCap = envll("D_SCAP", 1450) * W / max(E, 1LL);
    ll s = min({(ll)llround(sqrt((double)W)), sCap, W});
    if (s < 2) return;
    relS = s;
    relG = (W + s - 1) / s;
}
// finish-delay of worker j under relay: leaders finish early, members stream in
static ll relayDelay(ll j) {
    ll gi = (j - 1) / relS, pos = (j - 1) % relS;
    return gi + (pos == 0 ? 1 : pos);
}
static ll chunkBound(ll j) {  // b_j for j in 0..W (scanning workers only)
    ll W = K - 1 - pPRB;
    if (relS > 0) {
        static vector<ll> cum;
        if (cum.empty()) {
            cum.assign(W + 1, 0);
            ll A = max(4LL, E / W);
            for (ll t = 1; t <= W; t++) cum[t] = cum[t - 1] + A + relayDelay(t);
        }
        return (ll)((__int128)cum[j] * E / cum[W]);
    }
    ll A = max(4LL, E / W - (W - 1));
    // raw cumulative of A + 2(t-1) for t=1..j, then rescale to E
    ll cum = j * A + j * (j - 1);
    ll tot = W * A + W * (W - 1);
    return (ll)((__int128)cum * E / tot);
}

// pack a set of (worker -> chunk bits) into a 'B' message: 9-char present
// mask (bit j-1, 6 bits/char) then concatenated chunk bitmaps ascending
static string packB(const vector<pair<int, string>>& parts) {
    ll mask = 0;
    string allBits;
    for (auto& [j, b] : parts) mask |= 1LL << (j - 1);
    string body = "B";
    for (int c = 0; c < 9; c++) body += char(33 + ((mask >> (6 * c)) & 63));
    for (auto& [j, b] : parts) allBits += b;
    for (size_t i = 0; i < allBits.size(); i += 6) {
        int v = 0;
        for (size_t k = i; k < min(allBits.size(), i + 6); k++)
            v |= (allBits[k] - '0') << (5 - (k - i));
        body += char(33 + v);
    }
    return body;
}

static void workerStatic() {
    vector<int> ord = staticOrder();
    ll W = K - 1 - pPRB;
    vector<int> slice;
    if (envll("D_STRIDE", 1)) {
        // strided: every worker sweeps the diagonal strip together, so the
        // claim fires at the order's connection fraction (~0.68), not at 1.0
        for (ll i = ID - 1; i < E; i += W) slice.push_back(ord[i]);
    } else {
        for (ll i = chunkBound(ID - 1); i < chunkBound(ID); i++) slice.push_back(ord[i]);
    }
    ll S = slice.size(), wq = 0, fp = 0, reported = 0;
    string bits;
    const int F = 6;  // pipelined commands in flight
    deque<char> tags;
    deque<ll> qpos;  // slice positions of in-flight queries
    vector<signed char> bitOf(S, 2);  // 2 unknown, 3 in flight, 0/1 resolved
    // local map: provable walls / forced opens answered without a query
    vector<int> du(m * m);
    iota(du.begin(), du.end(), 0);
    auto find2 = [&](int x) {
        while (du[x] != x) x = du[x] = du[du[x]];
        return x;
    };
    vector<signed char> lu(m * m, 0), lo(m * m, 0);
    for (ll r = 0; r < m * m; r++)
        for (int d = 0; d < 4; d++)
            if (edgeId(r / m, r % m, d) >= 0) lu[r]++;
    auto localMark = [&](ll e, bool open) {
        EI x = edgeInfo(e);
        lu[x.u]--;
        lu[x.v]--;
        if (open) {
            lo[x.u]++;
            lo[x.v]++;
            du[find2(x.u)] = find2(x.v);
        }
    };
    auto flush = [&]() {
        while (fp < S && bitOf[fp] < 2) {
            bits += char('0' + bitOf[fp]);
            fp++;
        }
    };
    // stagger the first report so W same-cadence reports don't arrive in bursts
    ll firstP = max(8LL, pP * ID / max(W, 1LL));
    while (true) {
        bool wrote = true;
        while ((ll)tags.size() < F && wrote) {
            wrote = false;
            ll thr = reported == 0 ? firstP : pP;
            if ((ll)bits.size() >= thr || (fp == S && !bits.empty())) {
                string body = "R";
                body += enc(reported, 3);
                body += enc((ll)bits.size(), 2);
                for (size_t i = 0; i < bits.size(); i += 6) {
                    int v = 0;
                    for (size_t k = i; k < min(bits.size(), i + 6); k++)
                        v |= (bits[k] - '0') << (5 - (k - i));
                    body += char(33 + v);
                }
                wr("> 0 " + body);
                reported += (ll)bits.size();
                bits.clear();
                tags.push_back('r');
                wrote = true;
            } else if (wq < S) {
                // free-answer run: provable walls & forced opens skip the query
                while (wq < S) {
                    ll e = slice[wq];
                    EI x = edgeInfo(e);
                    if (find2(x.u) == find2(x.v)) {
                        bitOf[wq] = 0;
                        localMark(e, false);
                        wq++;
                        continue;
                    }
                    if ((lo[x.u] == 0 && lu[x.u] == 1) ||
                        (lo[x.v] == 0 && lu[x.v] == 1)) {
                        bitOf[wq] = 1;
                        localMark(e, true);
                        wq++;
                        continue;
                    }
                    break;
                }
                flush();
                wrote = true;
                if (wq < S) {
                    bitOf[wq] = 3;
                    EI x = edgeInfo(slice[wq]);
                    wr("? " + to_string(x.r) + " " + to_string(x.c));
                    qpos.push_back(wq);
                    wq++;
                    tags.push_back('q');
                }
            } else if (fp == S && bits.empty() && tags.empty()) {
                wr("halt");
                exit(0);
            }
        }
        string rep = rd();
        char t = tags.front();
        tags.pop_front();
        if (t == 'q') {
            ll p = qpos.front();
            qpos.pop_front();
            bitOf[p] = rep[0] == '1';
            localMark(slice[p], bitOf[p]);
            flush();
        }
    }
}

// ---------------- coordinator ----------------
#define CNEAR pC
#define KRINGS pKR

static vector<int> dsu, dsz;
static int find_(int x) {
    while (dsu[x] != x) x = dsu[x] = dsu[dsu[x]];
    return x;
}
static vector<vector<int>> compRooms;
static int rootS, rootG;
static vector<int> blobS, blobG;   // rooms of S/G comp in join order (landmarks)
static vector<int> dS, dG;         // ring-dist upper bounds (init: L1 to corners)
static vector<char> est;           // 0 unknown, 1 in-flight, 2 known
static vector<array<char, 4>> openAdj;  // revealed open passages per room
static vector<vector<int>> buckets;
static int curB = 0;
static ll dialCnt = 0;
static deque<int> hotQ;   // direct S<->G bridges
static deque<int> selfQ;  // dribbles the coordinator handles itself

static void dialPush(ll e, ll p) {
    p = max(0LL, min(p, (ll)buckets.size() - 1));
    buckets[p].push_back((int)e);
    curB = min(curB, (int)p);
    dialCnt++;
}

static ll dFarTo(const vector<int>& blob, ll i, ll j) {
    static const ll LMN = envll("D_LMN", 32);
    ll best = LLONG_MAX;
    ll n = blob.size(), stride = max(1LL, n / LMN);
    for (ll t = 0; t < n; t += stride) {
        ll r = blob[t];
        best = min(best, llabs(r / m - i) + llabs(r % m - j));
    }
    return best;
}

static void claimAndExit() {
    // BFS over revealed opens from start to goal
    int start = 0, goal = int(m * m - 1);
    vector<int> par(m * m, -2);
    par[start] = -1;
    deque<int> q{start};
    while (!q.empty()) {
        int x = q.front();
        q.pop_front();
        if (x == goal) break;
        ll i = x / m, j = x % m;
        for (int d = 0; d < 4; d++) {
            if (!openAdj[x][d]) continue;
            int n = int((i + DI[d]) * m + (j + DJ[d]));
            if (par[n] == -2) {
                par[n] = (x << 2) | d;
                q.push_back(n);
            }
        }
    }
    string seq;
    for (int cur = goal; par[cur] >= 0; cur = par[cur] >> 2) seq += DCH[par[cur] & 3];
    reverse(seq.begin(), seq.end());
    string out;
    for (char ch : seq) {
        out += ch;
        out += ch;
    }
    wr("! " + out);
    exit(0);
}

static void processResult(ll e, bool open);
// push edges of room r for side s (0=S) at ring level v; bridge-scan at v==0
static vector<signed char> inDir;  // per room: dir of the open edge it was entered by
static void pushEdgesSide(int s, int r, int v) {
    ll i = r / m, j = r % m;
    int otherRoot = s == 0 ? rootG : rootS;
    const vector<int>& otherBlob = s == 0 ? blobG : blobS;
    static ll corr = envll("D_CORR", 0);
    for (int d = 0; d < 4; d++) {
        ll e = edgeId(i, j, d);
        if (e < 0 || est[e] == 2) continue;
        int w = int((i + DI[d]) * m + (j + DJ[d]));
        if (v == 0) {
            int fw = find_(w), fr = find_(r);
            if (fw == fr) {
                processResult(e, false);  // same comp: guaranteed wall
                continue;
            }
            if (fw == otherRoot) {
                hotQ.push_back((int)e);  // direct S<->G bridge
                continue;
            }
        }
        if (est[e] == 0) {
            static ll bfsMode = envll("D_BFS", 0);
            static ll bfsSeq = 0;
            ll p;
            if (bfsMode) {
                // parallel-BFS: discovery order, no target bias — the whole
                // frontier is needed regardless, so staleness costs nothing
                p = min((bfsSeq++) >> 6, 2900LL);
            } else {
                p = (ll)CNEAR * v + dFarTo(otherBlob, i + DI[d], j + DJ[d]);
                if (v == 0 && !inDir.empty() && inDir[r] == d) p = max(0LL, p - corr);
            }
            dialPush(e, p);
        }
    }
}

static void relaxSide(int s, int r0, int v0) {
    vector<int>& dist = s == 0 ? dS : dG;
    if (dist[r0] <= v0) return;
    deque<pair<int, int>> q{{r0, v0}};
    dist[r0] = v0;
    while (!q.empty()) {
        auto [r, v] = q.front();
        q.pop_front();
        pushEdgesSide(s, r, v);
        if (v >= KRINGS) continue;
        ll i = r / m, j = r % m;
        for (int d = 0; d < 4; d++) {
            ll ni = i + DI[d], nj = j + DJ[d];
            if (ni < 0 || nj < 0 || ni >= m || nj >= m) continue;
            int n = int(ni * m + nj);
            if (dist[n] > v + 1) {
                dist[n] = v + 1;
                q.push_back({n, v + 1});
            }
        }
    }
}

static void processOpen(ll e) {
    EI x = edgeInfo(e);
    int u = x.u, v = x.v;
    int ru = find_(u), rv = find_(v);
    if (ru == rv) return;
    // record adjacency
    ll ui = u / m, uj = u % m, vi = v / m, vj = v % m;
    int d = vi == ui - 1 ? 0 : vj == uj - 1 ? 1 : vi == ui + 1 ? 2 : 3;
    openAdj[u][d] = 1;
    openAdj[v][d ^ 2] = 1;
    if (!inDir.empty()) {
        inDir[v] = (signed char)d;      // v entered via move d from u
        inDir[u] = (signed char)(d ^ 2);
    }
    bool sIn = ru == rootS || rv == rootS;
    bool gIn = ru == rootG || rv == rootG;
    if (sIn && gIn) claimAndExit();
    int sOther = sIn ? (ru == rootS ? rv : ru) : -1;  // comp that joins S
    int gOther = gIn ? (ru == rootG ? rv : ru) : -1;  // comp that joins G
    // physical union by comp size
    int a = ru, b = rv;
    if (compRooms[a].size() < compRooms[b].size()) swap(a, b);
    dsu[b] = a;
    vector<int> moved;
    moved.swap(compRooms[b]);
    auto roomsOf = [&](int root) -> vector<int>& {
        return root == b ? moved : compRooms[root];
    };
    if (sIn) {
        rootS = a;
        for (int r : roomsOf(sOther)) {
            blobS.push_back(r);
            relaxSide(0, r, 0);
        }
    }
    if (gIn) {
        rootG = a;
        for (int r : roomsOf(gOther)) {
            blobG.push_back(r);
            relaxSide(1, r, 0);
        }
    }
    compRooms[a].insert(compRooms[a].end(), moved.begin(), moved.end());
}

static vector<pair<int, int>> revealLog;  // (edge*2+bit, origin agent)
static int gOrigin = 0;
static vector<signed char> gUnk, gOpn;  // per-room unknown/open edge counts
static void processResult(ll e, bool open) {
    if (est[e] == 2) return;
    est[e] = 2;
    revealLog.push_back({int(e * 2 + open), gOrigin});
    EI x = edgeInfo(e);
    if (!gUnk.empty()) {
        gUnk[x.u]--;
        gUnk[x.v]--;
        if (open) {
            gOpn[x.u]++;
            gOpn[x.v]++;
        }
    }
    if (open) processOpen(e);
    if (gUnk.empty()) return;
    // degree rule: a room with no open yet and one unknown edge must open there
    for (int r : {x.u, x.v}) {
        if (gOpn[r] != 0 || gUnk[r] != 1) continue;
        ll i = r / m, j = r % m;
        for (int d = 0; d < 4; d++) {
            ll ee = edgeId(i, j, d);
            if (ee >= 0 && est[ee] != 2) {
                processResult(ee, true);
                break;
            }
        }
    }
}

// ---------------- relay worker (drain-bound small mazes) ----------------
static void workerRelay() {
    vector<int> ord = staticOrder();
    ll W = K - 1;
    vector<int> slice;
    for (ll i = chunkBound(ID - 1); i < chunkBound(ID); i++) slice.push_back(ord[i]);
    ll S = slice.size();
    // local map for free answers
    vector<int> du(m * m);
    iota(du.begin(), du.end(), 0);
    auto find2 = [&](int x) {
        while (du[x] != x) x = du[x] = du[du[x]];
        return x;
    };
    vector<signed char> lu(m * m, 0), lo(m * m, 0);
    for (ll r = 0; r < m * m; r++)
        for (int d = 0; d < 4; d++)
            if (edgeId(r / m, r % m, d) >= 0) lu[r]++;
    string myBits;
    for (ll p = 0; p < S; p++) {
        ll e = slice[p];
        EI x = edgeInfo(e);
        bool open;
        if (find2(x.u) == find2(x.v)) {
            open = false;
        } else if ((lo[x.u] == 0 && lu[x.u] == 1) ||
                   (lo[x.v] == 0 && lu[x.v] == 1)) {
            open = true;
        } else {
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            open = rd()[0] == '1';
        }
        myBits += open ? '1' : '0';
        lu[x.u]--;
        lu[x.v]--;
        if (open) {
            lo[x.u]++;
            lo[x.v]++;
            du[find2(x.u)] = find2(x.v);
        }
    }
    ll gi = (ID - 1) / relS, pos = (ID - 1) % relS;
    ll gsize = min(relS, W - gi * relS);
    vector<pair<int, string>> parts{{(int)ID, myBits}};
    if (pos != 0 || gsize == 1) {
        ll dst = pos == 0 ? 0 : 1 + gi * relS;
        wr("> " + to_string(dst) + " " + packB(parts));
        rd();
        wr("halt");
        exit(0);
    }
    // leader: gather members, then forward the merged group bitmap
    ll need = gsize - 1, waited = 0;
    while (need > 0 && waited < 3 * relS + 80) {
        wr("< ?");
        string rep = rd();
        waited++;
        if (rep[0] == '-') continue;
        size_t sp = rep.find(' ');
        const char* b = rep.c_str() + sp + 1;
        size_t blen = rep.size() - sp - 1;
        if (b[0] != 'B') continue;
        ll mask = 0;
        for (int c = 0; c < 9; c++) mask |= (ll)(b[1 + c] - 33) << (6 * c);
        string all;
        for (size_t i = 10; i < blen; i++) {
            int v = b[i] - 33;
            for (int t = 5; t >= 0; t--) all += char('0' + ((v >> t) & 1));
        }
        ll bitpos = 0;
        for (ll j = 1; j <= W; j++) {
            if (!((mask >> (j - 1)) & 1)) continue;
            ll cs = chunkBound(j) - chunkBound(j - 1);
            parts.push_back({(int)j, all.substr(bitpos, cs)});
            bitpos += cs;
            need--;
        }
    }
    sort(parts.begin(), parts.end());
    wr("> 0 " + packB(parts));
    rd();
    wr("halt");
    exit(0);
}

static void coordStatic() {
    ll W = K - 1;
    vector<int> ord = staticOrder();
    ll Wsc = W - pPRB;  // scanning workers; the rest are chain probers
    vector<vector<int>> slice(K);
    if (envll("D_STRIDE", 1)) {
        for (ll i = 0; i < E; i++) slice[1 + i % Wsc].push_back(ord[i]);
    } else {
        for (ll j = 1; j <= Wsc; j++)
            for (ll i = chunkBound(j - 1); i < chunkBound(j); i++)
                slice[j].push_back(ord[i]);
    }
    dsu.resize(m * m);
    iota(dsu.begin(), dsu.end(), 0);
    dsz.assign(m * m, 1);
    est.assign(E, 0);
    openAdj.assign(m * m, {0, 0, 0, 0});
    int startR = 0, goalR = int(m * m - 1);
    auto openEdge = [&](ll e) {
        EI x = edgeInfo(e);
        ll ui = x.u / m, uj = x.u % m, vi = x.v / m, vj = x.v % m;
        int d = vi == ui - 1 ? 0 : vj == uj - 1 ? 1 : vi == ui + 1 ? 2 : 3;
        openAdj[x.u][d] = 1;
        openAdj[x.v][d ^ 2] = 1;
        int ru = find_(x.u), rv = find_(x.v);
        if (ru != rv) {
            if (dsz[ru] < dsz[rv]) swap(ru, rv);
            dsu[rv] = ru;
            dsz[ru] += dsz[rv];
        }
        if (find_(startR) == find_(goalR)) claimAndExit();
    };
    // degree-rule deduction: per-room unknown/open counts over est-known edges
    vector<signed char> gu(m * m, 0), go(m * m, 0);
    for (ll r = 0; r < m * m; r++)
        for (int d = 0; d < 4; d++)
            if (edgeId(r / m, r % m, d) >= 0) gu[r]++;
    function<void(ll, bool)> known = [&](ll e, bool open) {
        if (est[e] == 2) return;
        est[e] = 2;
        EI x = edgeInfo(e);
        gu[x.u]--;
        gu[x.v]--;
        if (open) {
            go[x.u]++;
            go[x.v]++;
            openEdge(e);
        }
        for (int r : {x.u, x.v}) {
            if (go[r] != 0 || gu[r] != 1) continue;
            ll i = r / m, j = r % m;
            for (int d = 0; d < 4; d++) {
                ll ee = edgeId(i, j, d);
                if (ee >= 0 && est[ee] != 2) {
                    known(ee, true);
                    break;
                }
            }
        }
    };
    ll maxChunk = chunkBound(W) - chunkBound(W - 1);
    ll expected = maxChunk + pP + 400;
    ll scanEnd = maxChunk;  // when the last worker finishes querying
    ll turn = 0, sweepIdx = 0, sweep = 0;
    bool lastMiss = false;
    vector<char> freeW(K, 0);  // finished workers + dedicated probers
    for (ll j = W - pPRB + 1; j <= W; j++) freeW[j] = 1;
    ll stealCur = 0;
    ll lastChainT = 0;
    deque<ll> chainQ;  // unknown edges of the current best S-G chain
    auto chainProbeS = [&]() {
        vector<int> dist(m * m, INT_MAX), from(m * m, -1);
        deque<int> dq;
        dist[startR] = 0;
        dq.push_back(startR);
        while (!dq.empty()) {
            int x = dq.front();
            dq.pop_front();
            if (x == goalR) break;
            ll i = x / m, j = x % m;
            for (int d = 0; d < 4; d++) {
                ll e = edgeId(i, j, d);
                if (e < 0) continue;
                int nb = int((i + DI[d]) * m + (j + DJ[d]));
                int w;
                if (openAdj[x][d])
                    w = 0;
                else if (est[e] == 2 && !openAdj[x][d])
                    continue;  // known wall
                else
                    w = 1;
                if (dist[x] + w < dist[nb] && dist[x] + w <= 60) {
                    dist[nb] = dist[x] + w;
                    from[nb] = (x << 2) | d;
                    if (w == 0)
                        dq.push_front(nb);
                    else
                        dq.push_back(nb);
                }
            }
        }
        if (dist[goalR] > 45) return;
        chainQ.clear();
        for (int cur = goalR; from[cur] >= 0; cur = from[cur] >> 2) {
            int p = from[cur] >> 2, d = from[cur] & 3;
            if (!openAdj[p][d]) {
                ll e = edgeId(p / m, p % m, d);
                if (e >= 0 && est[e] == 0) chainQ.push_back(e);
            }
        }
    };
    vector<int> cand;  // unknown edges directly bridging comp(start) & comp(goal)
    ll candStamp = -64;
    while (true) {
        turn++;
        if (turn > expected && (sweep++ & 7) != 7) {
            // failsafe: a worker died or stalled — finish its edges ourselves
            while (sweepIdx < E && est[ord[sweepIdx]] == 2) sweepIdx++;
            if (sweepIdx < E) {
                ll e = ord[sweepIdx];
                EI x = edgeInfo(e);
                wr("? " + to_string(x.r) + " " + to_string(x.c));
                known(e, rd()[0] == '1');
                continue;
            }
        }
        // adaptive hot lane: on idle turns, self-query the frontier edge of
        // comp(start)/comp(goal) nearest the other comp — sews the path early
        if (envll("D_HOT", 0) && lastMiss) {
            int rs = find_(startR), rg = find_(goalR);
            ll bestE = -1, bestD = LLONG_MAX;
            for (int rep2 = 0; rep2 < 2; rep2++) {
                // bounding boxes of both comps via sampled openAdj rooms is
                // costly; approximate with corner anchors
            }
            // scan a window of unknown edges near both comps (cheap heuristic:
            // edges with exactly one endpoint in rs or rg comp)
            static ll hotCur = 0;
            int checked = 0;
            while (checked < 4000 && bestE < 0) {
                ll e = ord[hotCur % E];
                hotCur++;
                checked++;
                if (est[e] != 0) continue;
                EI x = edgeInfo(e);
                int fu = find_(x.u), fv = find_(x.v);
                bool su = fu == rs || fv == rs, sg = fu == rg || fv == rg;
                if (su && sg) {
                    bestE = e;  // direct bridge: take instantly
                    break;
                }
                if (su || sg) {
                    // distance of far room to the opposite corner as proxy
                    ll i = x.u / m, j = x.u % m;
                    ll d2 = su ? (2 * (m - 1) - i - j) : (i + j);
                    if (d2 < bestD) {
                        bestD = d2;
                        bestE = e;
                    }
                }
            }
            if (bestE >= 0) {
                EI x = edgeInfo(bestE);
                wr("? " + to_string(x.r) + " " + to_string(x.c));
                known(bestE, rd()[0] == '1');
                lastMiss = false;
                continue;
            }
        }
        // endgame: spend miss/tail turns probing S<->G bridge candidates instead
        // of waiting out a report period for the closing bit
        if (lastMiss || turn > scanEnd - 2 * pP) {
            if (turn - candStamp >= 32) {
                candStamp = turn;
                cand.clear();
                int rs = find_(startR), rg = find_(goalR);
                for (ll e = 0; e < E; e++) {
                    if (est[e] == 2) continue;
                    EI x = edgeInfo(e);
                    int fu = find_(x.u), fv = find_(x.v);
                    if ((fu == rs && fv == rg) || (fu == rg && fv == rs))
                        cand.push_back((int)e);
                }
            }
            bool did = false;
            while (!cand.empty()) {
                ll e = cand.back();
                cand.pop_back();
                if (est[e] != 0) continue;
                EI x = edgeInfo(e);
                int fu = find_(x.u), fv = find_(x.v);
                int rs = find_(startR), rg = find_(goalR);
                if (!((fu == rs && fv == rg) || (fu == rg && fv == rs))) continue;
                wr("? " + to_string(x.r) + " " + to_string(x.c));
                known(e, rd()[0] == '1');
                did = true;
                break;
            }
            if (did) {
                lastMiss = false;
                continue;
            }
        }
        // chain probing: refresh the best S-G chain periodically
        if (turn > scanEnd / 3 && turn - lastChainT >= 32) {
            lastChainT = turn;
            chainProbeS();
        }
        // work dispatch: chain batches first, then band-order stealing
        {
            ll fj = -1;
            for (ll j = 1; j < K; j++)
                if (freeW[j]) {
                    fj = j;
                    break;
                }
            if (fj > 0 && !chainQ.empty()) {
                string body = "A";
                int got = 0;
                while (!chainQ.empty() && got < 80) {
                    ll e = chainQ.front();
                    chainQ.pop_front();
                    if (est[e] != 0) continue;
                    est[e] = 1;
                    body += enc(e, 3);
                    got++;
                }
                if (got > 0) {
                    wr("> " + to_string(fj) + " " + body);
                    rd();
                    freeW[fj] = 0;
                    continue;
                }
            }
            if (fj > 0) {
                string body = "A";
                int got = 0;
                while (stealCur < E && got < 80) {
                    ll e = ord[stealCur];
                    if (est[e] != 0) {
                        stealCur++;
                        continue;
                    }
                    EI x = edgeInfo(e);
                    if (find_(x.u) == find_(x.v)) {
                        known(e, false);
                        stealCur++;
                        continue;
                    }
                    est[e] = 1;
                    body += enc(e, 3);
                    got++;
                    stealCur++;
                }
                if (got > 0) {
                    wr("> " + to_string(fj) + " " + body);
                    rd();
                    freeW[fj] = 0;
                    continue;
                }
            }
        }
        wr("< ?");
        string rep = rd();
        if (rep[0] == '-') {
            lastMiss = true;
            continue;
        }
        lastMiss = false;
        size_t sp = rep.find(' ');
        ll sj = stoll(rep.substr(0, sp));
        const char* b = rep.c_str() + sp + 1;
        if (b[0] == 'B') {  // relay bitmap: present-mask + packed chunks
            ll blen = (ll)rep.size() - (ll)sp - 1;
            ll mask = 0;
            for (int c = 0; c < 9; c++) mask |= (ll)(b[1 + c] - 33) << (6 * c);
            string all;
            for (ll i = 10; i < blen; i++) {
                int v = b[i] - 33;
                for (int t = 5; t >= 0; t--) all += char('0' + ((v >> t) & 1));
            }
            ll bitpos = 0;
            for (ll j = 1; j <= W; j++) {
                if (!((mask >> (j - 1)) & 1)) continue;
                for (ll i = chunkBound(j - 1); i < chunkBound(j); i++) {
                    known(ord[i], all[bitpos] == '1');
                    bitpos++;
                }
            }
        } else if (b[0] == 'D') {  // regional worker delta report
            ll blen = (ll)rep.size() - (ll)sp - 1;
            ll cnt = (blen - 1) / 3;
            for (ll i = 0; i < cnt; i++) {
                ll v = dec(b + 1 + 3 * i, 3);
                known(v >> 1, v & 1);
            }
            if ((blen - 1) % 3 == 1) freeW[sj] = 1;  // 'E': worker is idle
        } else {
            ll startIdx = dec(b + 1, 3);
            ll cnt = dec(b + 4, 2);
            for (ll i = 0; i < cnt; i++) {
                ll e = slice[sj][startIdx + i];
                int ch = b[6 + i / 6] - 33;
                known(e, (ch >> (5 - i % 6)) & 1);
            }
        }
    }
}

static int gLastPri = 0;
static bool dialPop(ll& eOut) {
    while (curB < (int)buckets.size()) {
        auto& bk = buckets[curB];
        while (!bk.empty()) {
            int e = bk.back();
            bk.pop_back();
            dialCnt--;
            if (est[e] != 0) continue;
            EI x = edgeInfo(e);
            if (find_(x.u) == find_(x.v)) {
                processResult(e, false);
                continue;
            }
            eOut = e;
            gLastPri = curB;
            return true;
        }
        curB++;
    }
    return false;
}

// dual-coordinator mode (pMODE==5): gMe = my agent id (0 or 1), gPeer = the
// other coordinator, gWorkers = worker ids I dispatch to
static ll gMe = 0, gPeer = -1;
static vector<ll> gWorkers;
static ll* selfProgress = nullptr;
static void selfQuery(ll e) {
    est[e] = 1;
    EI x = edgeInfo(e);
    wr("? " + to_string(x.r) + " " + to_string(x.c));
    gOrigin = 0;
    processResult(e, rd()[0] == '1');
    if (selfProgress) *selfProgress = 0;
}

static void coordinator() {
    ll W = K - 1;
    ll nE = pNE;
    ll targetOut = pTargetOut;
    int start = 0, goal = int(m * m - 1);
    dsu.resize(m * m);
    iota(dsu.begin(), dsu.end(), 0);
    compRooms.assign(m * m, {});
    for (int r = 0; r < m * m; r++) compRooms[r] = {r};
    rootS = start;
    rootG = goal;
    blobS = {start};
    blobG = {goal};
    dS.resize(m * m);
    dG.resize(m * m);
    for (ll i = 0; i < m; i++)
        for (ll j = 0; j < m; j++) {
            dS[i * m + j] = int(i + j);
            dG[i * m + j] = int((m - 1 - i) + (m - 1 - j));
        }
    est.assign(E, 0);
    openAdj.assign(m * m, {0, 0, 0, 0});
    // pure-greedy small-K: no speculative reservoir, no goal-side wave — the
    // oracle sim shows blanket breadth wastes 2-3x on easy mazes at tiny W
    bool pure = pMODE == 0 && W <= envll("D_PUREW", 5) && envll("D_PURE", 1);
    if (pMODE == 5) {
        gPeer = 1 - gMe;
        for (ll j = 2; j < K; j++)
            if ((ll)(j % 2) == gMe) gWorkers.push_back(j);
    } else {
        for (ll j = 1; j < K; j++) gWorkers.push_back(j);
    }
    ll Wme = (ll)gWorkers.size();
    ll lp = envll("D_LPURE", 0);  // lease-mode reservoir/G-wave removal test
    if (pMODE == 1 && lp) pure = true;
    bool keepG = pMODE == 1 && lp == 1;  // LPURE=1: drop reservoir only
    if (W >= envll("D_DEDW", 6)) {  // deduction perturbs tiny-K trajectories
        gUnk.assign(m * m, 0);
        gOpn.assign(m * m, 0);
        for (ll r = 0; r < m * m; r++)
            for (int d = 0; d < 4; d++)
                if (edgeId(r / m, r % m, d) >= 0) gUnk[r]++;
    }
    inDir.assign(m * m, -1);
    buckets.assign((CNEAR + 1) * 2 * m + 8, {});
        if (!pure) {
        // static reservoir: every edge priced off corner distances, layered
        // strictly BELOW adaptive pushes (D_ROFF) so breadth is fallback-only
        ll roff = envll("D_ROFF", 0);
        bool bfsRes = envll("D_BFS", 0);
        for (ll e = 0; e < E; e++) {
            if (pMODE == 5 && (e & 1) != gMe) continue;
            EI x = edgeInfo(e);
            if (bfsRes) {
                // BFS hybrid: diag-strip fallback priced below all BFS layers
                ll iu = x.u / m, ju = x.u % m, iv = x.v / m, jv = x.v % m;
                dialPush(e, 3000 + llabs(iu - ju) + llabs(iv - jv));
            } else {
                ll ds = min(dS[x.u], dS[x.v]), dg = min(dG[x.u], dG[x.v]);
                dialPush(e, roff + min((ll)CNEAR * ds + dg, (ll)CNEAR * dg + ds));
            }
        }
    }
    dS[start] = dG[goal] = 1;  // force relax to run
    if (pMODE == 5) {
        relaxSide(gMe == 0 ? 0 : 1, gMe == 0 ? start : goal, 0);
    } else {
        relaxSide(0, start, 0);
        if (!pure || keepG || envll("D_BFS", 0)) relaxSide(1, goal, 0);
    }

    vector<vector<ll>> asn(K);
    vector<ll> repc(K, 0);
    int missCooldown = 0;  // backoff when the readiness model misfires
    ll rrWorker = 0;
    ll sinceProgress = 0;  // stall detector: reclaims work from dead workers
    selfProgress = &sinceProgress;
    ll myTurn = 0;
    vector<ll> lastRep(K, 0), lastSend(K, 0);
    ll sjLast = -1;  // sender of the last processed report

    // ---- tile-lease state (pMODE==1) ----
    vector<int> tileOwner;
    vector<vector<ll>> sideline;  // dial entries parked while their tile is leased
    vector<ll> leaseOf;
    vector<int> bandOrd;
    ll bandCur = 0;
    if (pMODE == 1) {
        tileOwner.assign(G * G, -1);
        sideline.assign(G * G, {});
        leaseOf.assign(K, -1);
        bandOrd = staticOrder();
    }
    auto revealed = [&](int r) {
        return r == 0 || r == (int)(m * m - 1) ||
               openAdj[r][0] || openAdj[r][1] || openAdj[r][2] || openAdj[r][3];
    };
    auto priceEdge = [&](ll e) {
        EI x = edgeInfo(e);
        ll i = x.u / m, j = x.u % m;
        ll pS = (ll)CNEAR * min((ll)dS[x.u], (ll)dS[x.v]) + dFarTo(blobG, i, j);
        ll pG = (ll)CNEAR * min((ll)dG[x.u], (ll)dG[x.v]) + dFarTo(blobS, i, j);
        return min(pS, pG);
    };
    auto forTileEdges = [&](ll t, auto f) {  // edges touching tile t (interior twice)
        if (pTSH) {
            ll ta = t / G, tb = t % G;
            for (ll a = ta * pTS; a < min(2 * m - 1, (ta + 1) * pTS); a++)
                for (ll b = tb * pTS; b < min(2 * m - 1, (tb + 1) * pTS); b++) {
                    if ((a + b) % 2 != (m - 1) % 2) continue;
                    ll i = (a + b - (m - 1)) / 2, j = (a - b + (m - 1)) / 2;
                    if (i < 0 || j < 0 || i >= m || j >= m) continue;
                    for (int d = 0; d < 4; d++) {
                        ll e = edgeId(i, j, d);
                        if (e >= 0) f(e);
                    }
                }
            return;
        }
        ll ti = t / G, tj = t % G;
        for (ll i = ti * pTS; i < min(m, (ti + 1) * pTS); i++)
            for (ll j = tj * pTS; j < min(m, (tj + 1) * pTS); j++)
                for (int d = 0; d < 4; d++) {
                    ll e = edgeId(i, j, d);
                    if (e >= 0) f(e);
                }
    };
    auto releaseTile = [&](ll t) {
        if (t < 0) return;
        tileOwner[t] = -1;
        forTileEdges(t, [&](ll e) {
            if (est[e] == 1) {
                est[e] = 0;
                dialPush(e, priceEdge(e));
            }
        });
        for (ll e : sideline[t])
            if (est[e] == 0) dialPush(e, priceEdge(e));
        sideline[t].clear();
    };

    // ---- island-heads state (pMODE==3): chain start -> islands -> goal ----
    vector<ll> chainNode;
    vector<ll> headSeed(K, 0), headTgt(K, 0);
    vector<char> headSent(K, 2);  // 0 = needs (re)lease, 2 = active
    vector<ll> syncCur(K, 0);     // per-head cursor into revealLog for map-sync
    ll nextRecvTry = 0;
    if (pMODE == 3 && W > 0) {
        chainNode.push_back(0);
        for (ll h = 1; h <= W; h++) {
            ll r = (m - 1) * h / W;
            chainNode.push_back(h == W ? m * m - 1 : r * m + r);
        }
        for (ll j = 1; j < K; j++) {
            headSeed[j] = chainNode[j];
            headTgt[j] = chainNode[j - 1];  // each head grows its link backward
            headSent[j] = 0;
        }
    }

    auto tryRecv = [&]() -> bool {
        wr("< ?");
        string rep = rd();
        if (rep[0] == '-') {
            missCooldown = (int)envll("D_COOL", pMODE == 0 && K - 1 <= envll("D_PUREW", 5) ? 4 : 2);
            return false;
        }
        size_t sp = rep.find(' ');
        ll sj = stoll(rep.substr(0, sp));
        const char* b = rep.c_str() + sp + 1;
        if (b[0] == 'D') {  // lease-mode delta report
            ll blen = (ll)rep.size() - (ll)sp - 1;
            bool done = (blen - 1) % 3 == 1;
            ll cnt = (blen - 1) / 3;
            gOrigin = (int)sj;
            for (ll i = 0; i < cnt; i++) {
                ll v = dec(b + 1 + 3 * i, 3);
                processResult(v >> 1, v & 1);
            }
            if (done) {
                if (pMODE == 1 && leaseOf[sj] == UNB) {
                    leaseOf[sj] = -1;  // branch died: worker is free for regrant
                } else if (pMODE == 3) {
                    // island exhausted: reseed at the middle of the first
                    // unlinked chain link
                    for (size_t h = 0; h + 1 < chainNode.size(); h++) {
                        if (find_((int)chainNode[h]) != find_((int)chainNode[h + 1])) {
                            ll a = chainNode[h], b2 = chainNode[h + 1];
                            ll mi = (a / m + b2 / m) / 2, mj = (a % m + b2 % m) / 2;
                            headSeed[sj] = mi * m + mj;
                            headTgt[sj] = b2;
                            headSent[sj] = 0;
                            break;
                        }
                    }
                } else {
                    releaseTile(leaseOf[sj]);
                    leaseOf[sj] = -1;
                }
            }
        } else {
            ll startIdx = dec(b + 1, 3);
            ll cnt = dec(b + 4, 2);
            for (ll i = 0; i < cnt; i++) {
                if (startIdx + i >= (ll)asn[sj].size()) break;
                int ch = b[6 + i / 6] - 33;
                processResult(asn[sj][startIdx + i], (ch >> (5 - i % 6)) & 1);
            }
            repc[sj] = min(startIdx + cnt, (ll)asn[sj].size());
        }
        sinceProgress = 0;
        sjLast = sj;
        return true;
    };

    ll lastChain = 0;
    ll peerCur = 0;  // dual mode: revealLog cursor for peer sync
    // 0-1 BFS: if the min-unknown chain start->goal is short, hot-probe it
    auto chainProbe = [&]() {
        vector<int> dist(m * m, INT_MAX);
        vector<int> from(m * m, -1);
        deque<int> dq;
        dist[start] = 0;
        dq.push_back(start);
        int lim = 9;
        while (!dq.empty()) {
            int x = dq.front();
            dq.pop_front();
            if (x == goal) break;
            ll i = x / m, j = x % m;
            for (int d = 0; d < 4; d++) {
                ll e = edgeId(i, j, d);
                if (e < 0) continue;
                int nb = int((i + DI[d]) * m + (j + DJ[d]));
                int w;
                if (openAdj[x][d])
                    w = 0;
                else if (est[e] == 2)
                    continue;  // known wall
                else
                    w = 1;
                if (dist[x] + w < dist[nb] && dist[x] + w <= lim) {
                    dist[nb] = dist[x] + w;
                    from[nb] = (x << 2) | d;
                    if (w == 0)
                        dq.push_front(nb);
                    else
                        dq.push_back(nb);
                }
            }
        }
        if (dist[goal] > 8) return;
        for (int cur = goal; from[cur] >= 0; cur = from[cur] >> 2) {
            int p = from[cur] >> 2, d = from[cur] & 3;
            if (!openAdj[p][d]) {
                ll e = edgeId(p / m, p % m, d);
                if (e >= 0 && est[e] != 2) hotQ.push_back((int)e);
            }
        }
    };
    while (true) {
        myTurn++;
        if (myTurn - lastChain >= 32) {
            lastChain = myTurn;
            chainProbe();
        }
        if (++sinceProgress > 2000) {
            // a worker likely died: take back its in-flight work
            if (pMODE == 1) {
                for (ll j = 1; j < K; j++) {
                    releaseTile(leaseOf[j]);
                    leaseOf[j] = -1;
                }
            } else {
                for (ll j : gWorkers) {
                    for (ll t = repc[j]; t < (ll)asn[j].size(); t++) {
                        ll e = asn[j][t];
                        if (est[e] == 1) {
                            est[e] = 0;
                            dialPush(e, 0);
                        }
                    }
                    repc[j] = asn[j].size();
                }
            }
            sinceProgress = 0;
        }
        // 1. hot bridges: self-query, 1-turn latency
        bool didHot = false;
        while (!hotQ.empty()) {
            int e = hotQ.front();
            hotQ.pop_front();
            if (est[e] == 2) continue;
            EI x = edgeInfo(e);
            if (find_(x.u) == find_(x.v)) {
                processResult(e, false);
                continue;
            }
            selfQuery(e);
            didHot = true;
            break;
        }
        if (didHot) continue;
        // 2 (heads mode). autonomous island heads: initial leases & retargets
        if (pMODE == 3 && W > 0) {
            bool acted = false;
            for (ll j = 1; j < K && !acted; j++) {
                if (headSent[j]) continue;
                string seeds;
                ll si = headSeed[j] / m, sj2 = headSeed[j] % m;
                for (int d = 0; d < 4; d++) {
                    ll e = edgeId(si, sj2, d);
                    if (e >= 0 && est[e] == 0) seeds += enc(e, 3);
                }
                wr("> " + to_string(j) + " L" + enc(UNB, 2) + "T" + enc(headTgt[j], 3) + seeds);
                rd();
                headSent[j] = 2;
                acted = true;
            }
            for (ll j = 1; j < K && !acted; j++) {
                if (headSent[j] != 2) continue;
                int cur = find_((int)headSeed[j]);
                if (find_((int)headTgt[j]) != cur) continue;  // still tunneling
                // target reached its comp: aim at the nearest unmerged chain node
                ll nt = -1, bd = LLONG_MAX;
                for (ll nd : chainNode) {
                    if (find_((int)nd) == cur) continue;
                    ll d2 = llabs(nd / m - headSeed[j] / m) + llabs(nd % m - headSeed[j] % m);
                    if (d2 < bd) {
                        bd = d2;
                        nt = nd;
                    }
                }
                if (nt < 0 || nt == headTgt[j]) continue;
                headTgt[j] = nt;
                wr("> " + to_string(j) + " L" + enc(UNB, 2) + "T" + enc(nt, 3));
                rd();
                acted = true;
            }
            // map-sync: stream other heads' reveals so heads prune each other
            for (ll j = 1; j < K && !acted; j++) {
                if (headSent[j] != 2) continue;
                if ((ll)revealLog.size() - syncCur[j] < 120) continue;
                string body = "K";
                ll p = syncCur[j];
                int got = 0;
                for (; p < (ll)revealLog.size() && got < 83; p++) {
                    if (revealLog[p].second == (int)j) continue;
                    body += enc(revealLog[p].first, 3);
                    got++;
                }
                syncCur[j] = p;
                if (got == 0) continue;
                wr("> " + to_string(j) + " " + body);
                rd();
                acted = true;
            }
            if (acted) continue;
        }
        // 2 (branch-walker mode). grant the dial-top frontier edge as a
        // roaming seed: the worker follows that subtree at zero latency
        if (pMODE == 1 && W > 0 && envll("D_BR", 0)) {
            ll fj = -1;
            for (ll j = 1; j < K; j++)
                if (leaseOf[j] < 0) {
                    fj = j;
                    break;
                }
            if (fj > 0) {
                ll e;
                if (dialPop(e)) {
                    est[e] = 1;
                    EI x = edgeInfo(e);
                    // context: recent known results near the seed, so the walk
                    // doesn't re-query terrain other workers already covered
                    {
                        ll si = x.u / m, sj2 = x.u % m;
                        string ctx = "K";
                        int got = 0;
                        for (ll t = (ll)revealLog.size() - 1;
                             t >= 0 && got < 83 && t >= (ll)revealLog.size() - 4000; t--) {
                            ll ee = revealLog[t].first >> 1;
                            EI y = edgeInfo(ee);
                            if (llabs(y.u / m - si) + llabs(y.u % m - sj2) <= envll("D_CTXR", 10)) {
                                ctx += enc(revealLog[t].first, 3);
                                got++;
                            }
                        }
                        if (got > 8) {
                            wr("> " + to_string(fj) + " " + ctx);
                            rd();
                        }
                    }
                    string grant = "L" + enc(UNB, 2);
                    if (envll("D_BRL", 1)) {
                        // compact walk: target the seed's far room => the
                        // worker BFS-disks around the branch, no wandering
                        int fu = find_(x.u);
                        ll farRoom = (fu == rootS || fu == rootG) ? x.v : x.u;
                        grant += "T" + enc(farRoom, 3);
                    } else {
                        int fu = find_(x.u), fv = find_(x.v);
                        grant += (fu == rootG || fv == rootG) ? 'G' : 'S';
                    }
                    grant += enc(e, 3);
                    wr("> " + to_string(fj) + " " + grant);
                    rd();
                    leaseOf[fj] = UNB;  // marker: branch grant outstanding
                    continue;
                }
            }
        }
        // 2 (lease mode). lease the hottest free tile to a free worker
        if (pMODE == 1 && W > 0 && !envll("D_BR", 0)) {
            ll fj = -1;
            for (ll j = 1; j < K; j++)
                if (leaseOf[j] < 0) {
                    fj = j;
                    break;
                }
            if (fj > 0) {
                ll t = -1;
                bool scan = false;
                for (int guard = 0; guard < 2048 && t < 0; guard++) {
                    ll e;
                    if (!dialPop(e)) break;
                    EI x = edgeInfo(e);
                    ll tu = tileOfRoom(x.u), tv = tileOfRoom(x.v);
                    // lease the tile where expansion will happen
                    ll pick = (revealed(x.u) && !revealed(x.v)) ? tv : tu;
                    ll other = pick == tu ? tv : tu;
                    if (tileOwner[pick] == -1)
                        t = pick;
                    else if (tu != tv && tileOwner[other] == -1)
                        t = other;
                    else
                        sideline[pick].push_back(e);
                }
                if (t < 0 && !envll("D_NOSCAN", 0)) {
                    // speculative scan lease along the static band
                    for (int guard = 0; guard < 4096 && t < 0 && bandCur < E; guard++) {
                        ll e = bandOrd[bandCur];
                        if (est[e] != 0) {
                            bandCur++;
                            continue;
                        }
                        ll pt = tileOfRoom(edgeInfo(e).u);
                        if (tileOwner[pt] == -1) {
                            t = pt;
                            scan = true;
                        } else
                            bandCur++;
                    }
                }
                if (t >= 0 && !scan && envll("D_MINSEED", 0)) {
                    // adaptive lease must have at least one frontier seed; a
                    // seedless bounded lease silently full-scans the tile
                    int cnt = 0;
                    forTileEdges(t, [&](ll e) {
                        if (est[e] != 0 || cnt) return;
                        EI x = edgeInfo(e);
                        if (revealed(x.u) || revealed(x.v)) cnt++;
                    });
                    if (cnt == 0) t = -1;
                }
                if (t >= 0) {
                    string seeds;
                    int sc = 0;
                    ll sSide = 0, gSide = 0;
                    forTileEdges(t, [&](ll e) {
                        if (est[e] != 0) return;
                        if (!scan && sc < 82) {
                            EI x = edgeInfo(e);
                            bool ru = revealed(x.u), rv = revealed(x.v);
                            if (ru || rv) {
                                seeds += enc(e, 3);
                                sc++;
                                int rr = find_(ru ? x.u : x.v);
                                if (rr == rootS)
                                    sSide++;
                                else if (rr == rootG)
                                    gSide++;
                            }
                        }
                        est[e] = 1;
                    });
                    char side;
                    if (sc > 0 && sSide != gSide)
                        side = sSide > gSide ? 'S' : 'G';
                    else {
                        ll cd = (t / G) * pTS + (t % G) * pTS;
                        side = cd <= m - 1 ? 'S' : 'G';
                    }
                    wr("> " + to_string(fj) + " L" + enc(t, 2) + side + seeds);
                    rd();
                    leaseOf[fj] = t;
                    tileOwner[t] = (int)fj;
                    continue;
                }
            }
        }
        // 2b (dual). stream fresh reveals to the peer coordinator
        if (pMODE == 5 && (ll)revealLog.size() - peerCur >= 80) {
            string body = "D";
            ll p = peerCur;
            int got = 0;
            for (; p < (ll)revealLog.size() && got < 83; p++) {
                if (revealLog[p].second == (int)gPeer) continue;
                body += enc(revealLog[p].first, 3);
                got++;
            }
            peerCur = p;
            if (got > 0) {
                wr("> " + to_string(gPeer) + " " + body);
                rd();
                continue;
            }
        }
        // 2. batch to least-loaded worker
        if ((pMODE == 0 || pMODE == 5) && !gWorkers.empty() && dialCnt > 0) {
            ll bj = -1, bOut = LLONG_MAX;
            ll Wn = (ll)gWorkers.size();
            for (ll j0 = 0; j0 < Wn; j0++) {
                ll j = gWorkers[(rrWorker + j0) % Wn];
                ll out = (ll)asn[j].size() - repc[j];
                if (out < bOut) {
                    bOut = out;
                    bj = j;
                }
            }
            ll want = min(nE, targetOut - bOut);
            if (want >= 8) {
                string body = "A";
                ll got = 0, e;
                while (got < want && dialPop(e)) {
                    est[e] = 1;
                    asn[bj].push_back(e);
                    body += enc(e, 3);
                    got++;
                }
                if (got >= 8) {
                    rrWorker++;
                    if (repc[bj] == (ll)asn[bj].size() - got) lastSend[bj] = myTurn;
                    wr("> " + to_string(bj) + " " + body);
                    rd();
                    continue;
                }
                // dribble: undo assignment bookkeeping, self-handle
                for (ll t = 0; t < got; t++) {
                    selfQ.push_back((int)asn[bj].back());
                    asn[bj].pop_back();
                }
            }
        }
        // 3. receive reports
        ll unrep = 0;
        if (pMODE == 1) {
            for (ll j = 1; j < K; j++) unrep += leaseOf[j] >= 0;
        } else if (pMODE == 3) {
            for (ll j = 1; j < K; j++) unrep += headSent[j] == 2;
        } else {
            for (ll j : gWorkers) unrep += (ll)asn[j].size() - repc[j];
            if (pMODE == 5) unrep++;  // peer messages may arrive any time
        }
        if (pMODE == 3) {
            // reports arrive ~every 40/W turns; pace attempts to match
            if (unrep > 0 && myTurn >= nextRecvTry) {
                if (tryRecv())
                    nextRecvTry = myTurn;
                else
                    nextRecvTry = myTurn + max(2LL, 34 / max(W, 1LL));
                continue;
            }
        } else if (unrep > 0 && missCooldown == 0) {
            tryRecv();
            continue;
        }
        if (missCooldown > 0) missCooldown--;
        // 4. self work
        if (!selfQ.empty()) {
            int e = selfQ.front();
            selfQ.pop_front();
            if (est[e] != 1) continue;  // became known meanwhile
            EI x = edgeInfo(e);
            if (find_(x.u) == find_(x.v)) {
                processResult(e, false);
                continue;
            }
            est[e] = 0;
            selfQuery(e);
            continue;
        }
        {
            ll e;
            if (dialPop(e)) {
                selfQuery(e);
                continue;
            }
        }
        // 5. nothing else: poll
        missCooldown = 0;
        tryRecv();
    }
}


// ---------------- SYNC mode (pMODE==6): replicated-dial epochs ----------------
// Every agent replicates the identical exploration state (dial+DSU+deduction).
// Epochs: pop a shared plan from the dial (rank r takes slots r, A+r, ...),
// query, sync results up a binary tree as positional bitmaps, broadcast the
// merged epoch bitmap back down, apply identically. No dispatch messages.
static string packBits(const string& bits) {
    string out;
    for (size_t i = 0; i < bits.size(); i += 6) {
        int v = 0;
        for (size_t k = i; k < min(bits.size(), i + 6); k++)
            v |= (bits[k] - '0') << (5 - (k - i));
        out += char(33 + v);
    }
    return out;
}
static string unpackBits(const char* b, size_t nchars, size_t nbits) {
    string out;
    for (size_t i = 0; i < nchars && out.size() < nbits; i++) {
        int v = b[i] - 33;
        for (int t = 5; t >= 0 && out.size() < nbits; t--)
            out += char('0' + ((v >> t) & 1));
    }
    return out;
}

static void syncAgent() {
    ll A = K;
    ll me = ID;
    ll c1 = 2 * me + 1 < K ? 2 * me + 1 : -1;
    ll c2 = 2 * me + 2 < K ? 2 * me + 2 : -1;
    ll par2 = me > 0 ? (me - 1) / 2 : -1;
    // ---- replicated exploration state ----
    int start = 0, goal = int(m * m - 1);
    dsu.resize(m * m);
    iota(dsu.begin(), dsu.end(), 0);
    compRooms.assign(m * m, {});
    for (int r = 0; r < m * m; r++) compRooms[r] = {r};
    rootS = start;
    rootG = goal;
    blobS = {start};
    blobG = {goal};
    dS.resize(m * m);
    dG.resize(m * m);
    for (ll i = 0; i < m; i++)
        for (ll j = 0; j < m; j++) {
            dS[i * m + j] = int(i + j);
            dG[i * m + j] = int((m - 1 - i) + (m - 1 - j));
        }
    est.assign(E, 0);
    openAdj.assign(m * m, {0, 0, 0, 0});
    gUnk.assign(m * m, 0);
    gOpn.assign(m * m, 0);
    for (ll r = 0; r < m * m; r++)
        for (int d = 0; d < 4; d++)
            if (edgeId(r / m, r % m, d) >= 0) gUnk[r]++;
    inDir.assign(m * m, -1);
    buckets.assign((CNEAR + 1) * 2 * m + 8, {});
    // no static reservoir: epoch plans must stay frontier/ring-focused (the
    // reservoir floods 2600-edge plans with band junk => degenerates to a scan)
    dS[start] = dG[goal] = 1;
    relaxSide(0, start, 0);
    relaxSide(1, goal, 0);

    const ll L = max(8LL, min(96LL, envll("D_SL", 2600 / max(K, 1LL))));
    ll csm1 = 0, csm2 = 0;  // child subtree masks
    for (ll c : {c1, c2}) {
        if (c < 0) continue;
        ll msk = 0;
        deque<ll> q{c};
        while (!q.empty()) {
            ll x = q.front();
            q.pop_front();
            msk |= 1LL << x;
            if (2 * x + 1 < K) q.push_back(2 * x + 1);
            if (2 * x + 2 < K) q.push_back(2 * x + 2);
        }
        if (c == c1) csm1 = msk; else csm2 = msk;
    }

    ll mySubMask = 0;
    {
        deque<ll> q{me};
        while (!q.empty()) {
            ll x = q.front();
            q.pop_front();
            mySubMask |= 1LL << x;
            if (2 * x + 1 < K) q.push_back(2 * x + 1);
            if (2 * x + 2 < K) q.push_back(2 * x + 2);
        }
    }
    // pipelined epochs: EXEC(n) runs while sync(n-1) completes.
    struct Ep {
        vector<ll> plan;
        vector<string> bitsOf;
        ll mask = 0, fullMask = -1;   // fullMask valid only when wp == wt
        ll qi = 0;
        string myBits;
        bool upSent = false, fwd1 = false, fwd2 = false, applied = false;
        int polls = 0;
        int up1 = 0, ut1 = -1, up2 = 0, ut2 = -1;  // 'U' parts per child
        int wp = 0, wt = -1;                       // 'W' parts from parent
    };
    deque<Ep> eps;  // eps[0] = oldest un-applied epoch
    ll epBase = 0;  // epoch number of eps[0]

    auto planLenOf = [&](ll planSz, ll r) { return planSz / A + ((planSz % A) > r ? 1 : 0); };

    const ll PB = envll("D_PB", 40);  // priority band: skip deep-ring junk
    auto buildEpoch = [&]() {
        Ep ep;
        ep.bitsOf.assign(K, "");
        ll pri0 = -1;
        while ((ll)ep.plan.size() < A * L) {
            ll e = -1;
            while (!hotQ.empty()) {
                int cand = hotQ.front();
                hotQ.pop_front();
                if (est[cand] == 2) continue;
                EI x = edgeInfo(cand);
                if (find_(x.u) == find_(x.v)) {
                    processResult(cand, false);
                    continue;
                }
                e = cand;
                break;
            }
            if (e < 0) {
                if (!dialPop(e)) break;
                if (pri0 < 0) pri0 = gLastPri;
                if (gLastPri > pri0 + PB && (ll)ep.plan.size() >= A) {
                    // past the useful band: return it and stop
                    est[e] = 0;
                    dialPush(e, gLastPri);
                    break;
                }
            }
            est[e] = 1;
            ep.plan.push_back(e);
        }
        eps.push_back(move(ep));
    };

    auto applyEpoch = [&](Ep& ep) {
        ll fm = ep.fullMask;  // authoritative applied set (never local guess)
        for (ll i = 0; i < (ll)ep.plan.size(); i++) {
            ll r = i % A, idx = i / A, e = ep.plan[i];
            if (((fm >> r) & 1) && idx < (ll)ep.bitsOf[r].size()) {
                if (est[e] != 2) processResult(e, ep.bitsOf[r][idx] == '1');
            } else if (est[e] == 1) {
                est[e] = 0;
                dialPush(e, 0);
            }
        }
        ep.applied = true;
    };

    // tag 'U': headerMask == carriedMask (subtree bits). tag 'W': headerMask =
    // the authoritative applied-set; carried bits omit the recipient's subtree
    // (recipient substitutes its local copies — sound because any subtree bits
    // the root knows passed through the recipient).
    auto sendBits = [&](Ep& ep, ll epn, ll dst, char tag, ll headerMask, ll carryMask) {
        vector<ll> masks;
        ll cur = 0, bits = 0;
        for (ll r = 0; r < K; r++) {
            if (!((carryMask >> r) & 1)) continue;
            ll n2 = planLenOf((ll)ep.plan.size(), r);
            if (bits + n2 > 1380 && cur) {
                masks.push_back(cur);
                cur = 0;
                bits = 0;
            }
            cur |= 1LL << r;
            bits += n2;
        }
        if (cur || masks.empty()) masks.push_back(cur);
        for (size_t p = 0; p < masks.size(); p++) {
            string bb;
            for (ll r = 0; r < K; r++)
                if ((masks[p] >> r) & 1) bb += ep.bitsOf[r];
            string body;
            body += tag;
            body += char(33 + (int)(epn % 94));
            body += char(33 + (int)masks.size());
            body += char(33 + (int)p);
            // header mask: for the LAST part of a 'W', send headerMask bits not
            // covered by carried parts as a separate applied-only annotation is
            // not needed — header mask per part = carried part mask, except 'W'
            // final part ORs in (headerMask & ~carryMask) so the union over
            // parts equals the applied set.
            ll hm = masks[p];
            if (tag == 'W' && p + 1 == masks.size()) hm |= headerMask & ~carryMask;
            for (int t = 0; t < 9; t++) body += char(33 + ((hm >> (6 * t)) & 63));
            body += packBits(bb);
            wr("> " + to_string(dst) + " " + body);
            rd();
        }
    };

    // buffered receive: pull one message from src, file it into its epoch
    auto pullFrom = [&](ll src2) -> bool {
        wr("< " + to_string(src2));
        string rep = rd();
        if (rep[0] == '-') return false;
        size_t sp = rep.find(' ');
        const char* b = rep.c_str() + sp + 1;
        if (b[0] != 'U' && b[0] != 'W') return true;
        int nparts = b[2] - 33;
        int epn94 = b[1] - 33;
        ll idxEp = -1;
        for (ll t = 0; t < (ll)eps.size(); t++)
            if ((int)((epBase + t) % 94) == epn94) {
                idxEp = t;
                break;
            }
        if (idxEp < 0) return true;  // stale epoch, drop
        Ep& ep = eps[idxEp];
        ll cm = 0;
        for (int t = 0; t < 9; t++) cm |= (ll)(b[4 + t] - 33) << (6 * t);
        // carried bits: for 'W', my-subtree ranks are never carried (local copies)
        ll carry = b[0] == 'W' ? (cm & ~mySubMask) : cm;
        ll nb = 0;
        for (ll r = 0; r < K; r++)
            if ((carry >> r) & 1) nb += planLenOf((ll)ep.plan.size(), r);
        string all = unpackBits(b + 13, rep.size() - sp - 14, nb);
        ll pos = 0;
        for (ll r = 0; r < K; r++)
            if ((carry >> r) & 1) {
                ll n2 = planLenOf((ll)ep.plan.size(), r);
                ep.bitsOf[r] = all.substr(pos, n2);
                pos += n2;
            }
        if (b[0] == 'U') {
            ep.mask |= cm;
            if (src2 == c1) {
                ep.ut1 = nparts;
                ep.up1++;
            } else if (src2 == c2) {
                ep.ut2 = nparts;
                ep.up2++;
            }
        } else {
            ep.fullMask = (ep.fullMask < 0 ? 0 : ep.fullMask) | cm;
            ep.wt = nparts;
            ep.wp++;
        }
        return true;
    };

    buildEpoch();
    while (true) {
        // ---- one command per loop iteration, priority to oldest epoch's sync ----
        Ep& old2 = eps[0];
        ll oldN = epBase;
        bool oldExecDone = old2.qi * A + me >= (ll)old2.plan.size() || (ll)old2.myBits.size() >= planLenOf((ll)old2.plan.size(), me);
        if (oldExecDone && (ll)old2.myBits.size() > 0 && old2.bitsOf[me].empty()) {
            old2.bitsOf[me] = old2.myBits;
            old2.mask |= 1LL << me;
        }
        // gather children for oldest epoch (all parts of each child's 'U')
        bool needC1 = c1 >= 0 && (old2.ut1 < 0 || old2.up1 < old2.ut1);
        bool needC2 = c2 >= 0 && (old2.ut2 < 0 || old2.up2 < old2.ut2);
        if (oldExecDone && (needC1 || needC2) && old2.polls < 400) {
            old2.polls++;
            pullFrom(needC1 ? c1 : c2);
            continue;
        }
        // send up once
        if (oldExecDone && !old2.upSent && par2 >= 0) {
            sendBits(old2, oldN, par2, 'U', old2.mask, old2.mask);
            old2.upSent = true;
            continue;
        }
        if (oldExecDone && par2 < 0 && old2.fullMask < 0)
            old2.fullMask = old2.mask;  // root: my gathered set IS the applied set
        // wait for the full broadcast — no local fallback: determinism over liveness
        if (oldExecDone && old2.upSent && (old2.wt < 0 || old2.wp < old2.wt)) {
            pullFrom(par2);
            continue;
        }
        // forward down: header = applied set, bits = complement of child subtree
        if (old2.fullMask >= 0 && c1 >= 0 && !old2.fwd1) {
            sendBits(old2, oldN, c1, 'W', old2.fullMask, old2.fullMask & ~csm1);
            old2.fwd1 = true;
            continue;
        }
        if (old2.fullMask >= 0 && c2 >= 0 && !old2.fwd2) {
            sendBits(old2, oldN, c2, 'W', old2.fullMask, old2.fullMask & ~csm2);
            old2.fwd2 = true;
            continue;
        }
        // apply + retire oldest; plan a fresh epoch (pipeline depth 2)
        if (old2.fullMask >= 0) {
            applyEpoch(old2);
            eps.pop_front();
            epBase++;
            buildEpoch();
            continue;  // no command spent... must not loop without commands! fallthrough below
        }
        // ---- execute newest epoch queries ----
        Ep& cur2 = eps.back();
        ll myN = planLenOf((ll)cur2.plan.size(), me);
        if ((ll)cur2.myBits.size() < myN) {
            ll i = (ll)cur2.myBits.size() * A + me;
            EI x = edgeInfo(cur2.plan[i]);
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            cur2.myBits += rd()[0] == '1' ? '1' : '0';
            continue;
        }
        if (eps.size() == 1 && (ll)cur2.myBits.size() >= myN) {
            if (cur2.plan.empty()) {
                wr(".");
                rd();
            } else {
                buildEpoch();  // pipeline: execute epoch n+1 while n syncs
            }
            continue;
        }
        // nothing to do: idle poll toward parent/children for progress
        if (par2 >= 0)
            pullFrom(par2);
        else if (c1 >= 0)
            pullFrom(c1);
        else {
            wr(".");
            rd();
        }
    }
}


// ---------------- STREAM mode (pMODE==7): ownership-hash autonomous explorers -
// No plans, no barriers, no determinism requirement. Each agent explores the
// shared opens-map with a local frontier, but queries ONLY edges it owns
// (e % K == ID) — zero-coordination dedup. Opens flood the binary tree
// (forward to every neighbor except the source); walls stay local (DSU pruning
// needs opens only). First agent whose map connects start-goal claims.
static void streamAgent() {
    ll me = ID;
    ll nb[3] = {me > 0 ? (me - 1) / 2 : -1, 2 * me + 1 < K ? 2 * me + 1 : -1,
                2 * me + 2 < K ? 2 * me + 2 : -1};
    int start = 0, goal = int(m * m - 1);
    vector<char> lkn(E, 0);  // 0 unknown, 1 open, 2 wall (local only)
    vector<char> inMap(m * m, 0);
    vector<int> du(m * m);
    iota(du.begin(), du.end(), 0);
    auto find2 = [&](int x) {
        while (du[x] != x) x = du[x] = du[du[x]];
        return x;
    };
    vector<array<char, 4>> adj(m * m, {0, 0, 0, 0});
    priority_queue<pair<ll, ll>, vector<pair<ll, ll>>, greater<>> fr;
    deque<int> outQ[3];
    bool claimed = false;

    const ll P2 = envll("D_P2", 6);  // ring-2 speculation penalty
    auto pushRoom = [&](int room) {
        if (inMap[room]) return;
        inMap[room] = 1;
        ll i = room / m, j = room % m;
        for (int d = 0; d < 4; d++) {
            ll e = edgeId(i, j, d);
            if (e < 0 || lkn[e]) continue;
            ll ni = i + DI[d], nj = j + DJ[d];
            ll pri = min(2 * (m - 1) - ni - nj, ni + nj);
            if ((e % K) == me) fr.push({pri, e});
            // ring-2: my owned edges one step beyond this frontier edge —
            // hides propagation latency (corridors advance 2 per round trip)
            for (int d2 = 0; d2 < 4; d2++) {
                ll e2 = edgeId(ni, nj, d2);
                if (e2 < 0 || e2 == e || lkn[e2] || (e2 % K) != me) continue;
                fr.push({pri + P2, e2});
            }
        }
    };
    auto applyOpen = [&](ll e, int from) {  // from: -1 own, else neighbor index
        if (lkn[e] == 1) return;
        lkn[e] = 1;
        EI x = edgeInfo(e);
        ll ui = x.u / m, uj = x.u % m, vi = x.v / m, vj = x.v % m;
        int d = vi == ui - 1 ? 0 : vj == uj - 1 ? 1 : vi == ui + 1 ? 2 : 3;
        adj[x.u][d] = 1;
        adj[x.v][d ^ 2] = 1;
        du[find2(x.u)] = find2(x.v);
        for (int t = 0; t < 3; t++)
            if (nb[t] >= 0 && t != from) outQ[t].push_back((int)e);
        pushRoom(x.u);
        pushRoom(x.v);
        if (find2(start) == find2(goal)) claimed = true;
    };
    auto doClaim = [&]() {
        vector<int> par3(m * m, -2);
        par3[start] = -1;
        deque<int> q{start};
        while (!q.empty()) {
            int x = q.front();
            q.pop_front();
            if (x == goal) break;
            ll i = x / m, j = x % m;
            for (int d = 0; d < 4; d++) {
                if (!adj[x][d]) continue;
                int n2 = int((i + DI[d]) * m + (j + DJ[d]));
                if (par3[n2] == -2) {
                    par3[n2] = (x << 2) | d;
                    q.push_back(n2);
                }
            }
        }
        string seq2;
        for (int cur = goal; par3[cur] >= 0; cur = par3[cur] >> 2)
            seq2 += DCH[par3[cur] & 3];
        reverse(seq2.begin(), seq2.end());
        string out;
        for (char ch : seq2) {
            out += ch;
            out += ch;
        }
        wr("! " + out);
        exit(0);
    };

    ll turn2 = 0, lastPoll[3] = {0, 0, 0}, gap[3] = {2, 2, 2};
    ll lastSend[3] = {0, 0, 0};
    const ll FLUSH = envll("D_FLUSH", 42);
    const ll AGE = envll("D_AGE", 8);
    pushRoom(start);
    pushRoom(goal);
    while (true) {
        turn2++;
        if (claimed) doClaim();
        // 1. flush a full or aging buffer (age valve keeps opens flowing early)
        int fq = -1;
        for (int t = 0; t < 3; t++)
            if (nb[t] >= 0 && !outQ[t].empty() &&
                ((ll)outQ[t].size() >= FLUSH || turn2 - lastSend[t] >= AGE))
                fq = t;
        // 2. poll a due neighbor
        int pq2 = -1;
        for (int t = 0; t < 3; t++)
            if (nb[t] >= 0 && turn2 - lastPoll[t] >= gap[t] && pq2 < 0) pq2 = t;
        if (fq >= 0) {
            lastSend[fq] = turn2;
            string body = "D";
            int got = 0;
            while (!outQ[fq].empty() && got < 84) {
                body += enc((ll)outQ[fq].front() * 2 + 1, 3);
                outQ[fq].pop_front();
                got++;
            }
            wr("> " + to_string(nb[fq]) + " " + body);
            rd();
            continue;
        }
        if (pq2 >= 0) {
            lastPoll[pq2] = turn2;
            wr("< " + to_string(nb[pq2]));
            string rep = rd();
            if (rep[0] == '-') {
                gap[pq2] = min(12LL, gap[pq2] + 2);
            } else {
                gap[pq2] = 2;
                size_t sp = rep.find(' ');
                const char* b = rep.c_str() + sp + 1;
                size_t blen = rep.size() - sp - 1;
                if (b[0] == 'D')
                    for (size_t p = 1; p + 3 <= blen; p += 3)
                        applyOpen(dec(b + p, 3) >> 1, pq2);
            }
            continue;
        }
        // 3. own exploration
        bool didq = false;
        while (!fr.empty()) {
            auto [p2, e] = fr.top();
            fr.pop();
            if (lkn[e]) continue;
            EI x = edgeInfo(e);
            if (find2(x.u) == find2(x.v)) {
                lkn[e] = 2;  // provable wall, free
                continue;
            }
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            bool open = rd()[0] == '1';
            if (open)
                applyOpen(e, -1);
            else
                lkn[e] = 2;
            didq = true;
            break;
        }
        if (didq) continue;
        // 4. idle: flush any nonempty buffer, else poll round-robin anyway
        int nq = -1;
        for (int t = 0; t < 3; t++)
            if (nb[t] >= 0 && !outQ[t].empty()) nq = t;
        if (nq >= 0) {
            string body = "D";
            int got = 0;
            while (!outQ[nq].empty() && got < 84) {
                body += enc((ll)outQ[nq].front() * 2 + 1, 3);
                outQ[nq].pop_front();
                got++;
            }
            wr("> " + to_string(nb[nq]) + " " + body);
            rd();
        } else if (pq2 < 0 && nb[0] >= 0) {
            lastPoll[0] = turn2;
            wr("< " + to_string(nb[0]));
            string rep = rd();
            if (rep[0] != '-') {
                gap[0] = 2;
                size_t sp = rep.find(' ');
                const char* b = rep.c_str() + sp + 1;
                size_t blen = rep.size() - sp - 1;
                if (b[0] == 'D')
                    for (size_t p = 1; p + 3 <= blen; p += 3)
                        applyOpen(dec(b + p, 3) >> 1, 0);
            }
        } else {
            wr(".");
            rd();
        }
    }
}

int main() {
    {
        string st = rd();
        sscanf(st.c_str(), "%lld %lld %lld %lld", &N, &K, &ID, &MSGLEN);
    }
    m = (N + 1) / 2;
    HM = m * (m - 1);
    E = 2 * HM;
    initParams();
    initRelay();
    G = (pTSH ? 2 * m : m + pTS - 1) / pTS + 1;
    if (pMODE == 6 && N > 1) {
        syncAgent();
        return 0;
    }
    if (pMODE == 7 && N > 1) {
        streamAgent();
        return 0;
    }
    if (pMODE == 5 && !pStatic && ID == 1 && N > 1) {
        gMe = 1;
        coordinator();
        return 0;
    }
    if (ID != 0) {
        if (pStatic) {
            if (relS > 0)
                workerRelay();
            else if (pPRB > 0 && ID > K - 1 - pPRB)
                proberLoop();
            else if (envll("D_RG", 0) && E / max(K - 1, 1LL) >= envll("D_RGMIN", 400))
                workerRegional();
            else
                workerStatic();
        }
        else if (pMODE == 1 || pMODE == 3)
            workerLease();
        else
            worker();
        return 0;
    }
    if (N == 1) {
        wr("!");
        return 0;
    }
    if (pStatic)
        coordStatic();
    else
        coordinator();
    return 0;
}
