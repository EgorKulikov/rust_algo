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
static ll pTS;        // lease tile size (rooms per side)
static ll pMODE;      // adaptive dispatch: 0 = edge batches, 1 = tile leases
static void initParams() {
    ll W = max(K - 1, 1LL);
    pNE = min({(MSGLEN - 1) / 3, 84LL, envll("D_NE", 84)});
    ll nE = pNE;
    pR = max(envll("D_RMIN", 16), min({160LL, envll("D_RMUL", 2) * W, (MSGLEN - 6) * 6}));
    pTargetOut = min(2 * nE, max(2 * pR, envll("D_TMIN", 48)));
    pC = envll("D_C", 3);
    pKR = envll("D_KR", 4);
    pStatic = W >= envll("D_WTH", 22);
    pP = min(max(envll("D_P", 56), (10 * W + 7) / 8), (MSGLEN - 6) * 6);
    pTS = max(4LL, envll("D_TS", 10));
    pMODE = envll("D_MODE", 2);
    if (pMODE == 2)  // auto: tile leases pay at high worker counts & tiny mazes
        pMODE = (W >= envll("D_LW", 15) || m <= envll("D_LM", 60)) ? 1 : 0;
}
static ll G;  // tiles per side
static inline ll tileOfRoom(ll room) { return (room / m) / pTS * G + (room % m) / pTS; }

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
    auto poll = [&]() {
        wr("< 0");
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
            wr("> 0 " + body);
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
            wr("? " + to_string(x.r) + " " + to_string(x.c));
            bits += (rd()[0] == '1') ? '1' : '0';
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
            results += enc(e * 2 + open, 3);
            pend++;
            if (open) {
                du[find2(x.u)] = find2(x.v);
                if (myTile == UNB || tileOfRoom(x.u) == myTile) addRoom(x.u);
                if (myTile == UNB || tileOfRoom(x.v) == myTile) addRoom(x.v);
            }
        } else {
            bool done = fr.empty();
            string body = "D" + results;
            if (done) body += 'E';
            wr("> 0 " + body);
            results.clear();
            pend = 0;
            rd();
            if (done) myTile = -1;
        }
    }
}

// ---------------- static full-scan mode (large K) ----------------
// Global band order: edges sorted by (L1 to start corner + L1 to goal corner),
// identical in every agent. Worker j scans order[j-1], order[j-1+W], ... and
// streams packed bitmap reports; the coordinator only ingests and claims.
static vector<int> staticOrder() {
    vector<int> key(E);
    int maxK = 0;
    for (ll e = 0; e < E; e++) {
        EI x = edgeInfo(e);
        ll iu = x.u / m, ju = x.u % m, iv = x.v / m, jv = x.v % m;
        ll ds = min(iu + ju, iv + jv);
        ll dg = min((m - 1 - iu) + (m - 1 - ju), (m - 1 - iv) + (m - 1 - jv));
        key[e] = int(ds + dg);
        maxK = max(maxK, key[e]);
    }
    vector<int> cnt(maxK + 2, 0);
    for (ll e = 0; e < E; e++) cnt[key[e] + 1]++;
    for (int k = 1; k <= maxK + 1; k++) cnt[k] += cnt[k - 1];
    vector<int> ord(E);
    for (ll e = 0; e < E; e++) ord[cnt[key[e]]++] = (int)e;
    return ord;
}

static void workerStatic() {
    vector<int> ord = staticOrder();
    ll W = K - 1;
    vector<int> slice;
    for (ll i = ID - 1; i < E; i += W) slice.push_back(ord[i]);
    ll S = slice.size(), wq = 0, rq = 0, reported = 0;
    string bits;
    const int F = 6;  // pipelined commands in flight
    deque<char> tags;
    // stagger the first report so W same-cadence reports don't arrive in bursts
    ll firstP = max(8LL, pP * ID / max(W, 1LL));
    while (true) {
        bool wrote = true;
        while ((ll)tags.size() < F && wrote) {
            wrote = false;
            ll thr = reported == 0 ? firstP : pP;
            if ((ll)bits.size() >= thr || (rq == S && !bits.empty())) {
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
                EI x = edgeInfo(slice[wq]);
                wr("? " + to_string(x.r) + " " + to_string(x.c));
                wq++;
                tags.push_back('q');
                wrote = true;
            } else if (rq == S && bits.empty() && tags.empty()) {
                wr("halt");
                exit(0);
            }
        }
        string rep = rd();
        char t = tags.front();
        tags.pop_front();
        if (t == 'q') {
            bits += rep[0] == '1' ? '1' : '0';
            rq++;
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
    ll best = LLONG_MAX;
    ll n = blob.size(), stride = max(1LL, n / 32);
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
                est[e] = 2;  // same comp: guaranteed wall
                continue;
            }
            if (fw == otherRoot) {
                hotQ.push_back((int)e);  // direct S<->G bridge
                continue;
            }
        }
        if (est[e] == 0) {
            ll p = (ll)CNEAR * v + dFarTo(otherBlob, i + DI[d], j + DJ[d]);
            if (v == 0 && !inDir.empty() && inDir[r] == d) p = max(0LL, p - corr);
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
static void processResult(ll e, bool open) {
    if (est[e] == 2) return;
    est[e] = 2;
    revealLog.push_back({int(e * 2 + open), gOrigin});
    if (open) processOpen(e);
}

static void coordStatic() {
    ll W = K - 1;
    vector<int> ord = staticOrder();
    vector<vector<int>> slice(K);
    for (ll i = 0; i < E; i++) slice[1 + i % W].push_back(ord[i]);
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
    ll expected = (E + W - 1) / W + pP + 400;
    ll scanEnd = (E + W - 1) / W;  // when workers finish querying
    ll turn = 0, sweepIdx = 0, sweep = 0;
    bool lastMiss = false;
    vector<int> cand;  // unknown edges directly bridging comp(start) & comp(goal)
    ll candStamp = -64;
    while (true) {
        turn++;
        if (turn > expected && (sweep++ & 7) != 7) {
            // failsafe: a worker died or stalled — finish its edges ourselves
            while (sweepIdx < E && est[ord[sweepIdx]] != 0) sweepIdx++;
            if (sweepIdx < E) {
                ll e = ord[sweepIdx];
                est[e] = 2;
                EI x = edgeInfo(e);
                wr("? " + to_string(x.r) + " " + to_string(x.c));
                if (rd()[0] == '1') openEdge(e);
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
                    if (est[e] != 0) continue;
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
                est[e] = 2;
                wr("? " + to_string(x.r) + " " + to_string(x.c));
                if (rd()[0] == '1') openEdge(e);
                did = true;
                break;
            }
            if (did) {
                lastMiss = false;
                continue;
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
        ll startIdx = dec(b + 1, 3);
        ll cnt = dec(b + 4, 2);
        for (ll i = 0; i < cnt; i++) {
            ll e = slice[sj][startIdx + i];
            est[e] = 2;
            int ch = b[6 + i / 6] - 33;
            if ((ch >> (5 - i % 6)) & 1) openEdge(e);
        }
    }
}

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
                est[e] = 2;
                continue;
            }
            eOut = e;
            return true;
        }
        curB++;
    }
    return false;
}

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
    inDir.assign(m * m, -1);
    buckets.assign((CNEAR + 1) * 2 * m + 8, {});
    // static reservoir: every edge priced off corner distances
    for (ll e = 0; e < E; e++) {
        EI x = edgeInfo(e);
        ll ds = min(dS[x.u], dS[x.v]), dg = min(dG[x.u], dG[x.v]);
        dialPush(e, min((ll)CNEAR * ds + dg, (ll)CNEAR * dg + ds));
    }
    dS[start] = dG[goal] = 1;  // force relax to run
    relaxSide(0, start, 0);
    relaxSide(1, goal, 0);

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
            missCooldown = (int)envll("D_COOL", 2);
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
                if (pMODE == 3) {
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

    while (true) {
        myTurn++;
        if (++sinceProgress > 2000) {
            // a worker likely died: take back its in-flight work
            if (pMODE == 1) {
                for (ll j = 1; j < K; j++) {
                    releaseTile(leaseOf[j]);
                    leaseOf[j] = -1;
                }
            } else {
                for (ll j = 1; j < K; j++) {
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
                est[e] = 2;
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
        // 2 (lease mode). lease the hottest free tile to a free worker
        if (pMODE == 1 && W > 0) {
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
                if (t < 0) {
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
        // 2. batch to least-loaded worker
        if (pMODE == 0 && W > 0 && dialCnt > 0) {
            ll bj = -1, bOut = LLONG_MAX;
            for (ll j0 = 0; j0 < W; j0++) {
                ll j = 1 + (rrWorker + j0) % W;
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
                    rrWorker = bj % max(W, 1LL);
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
            for (ll j = 1; j < K; j++) unrep += (ll)asn[j].size() - repc[j];
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
                est[e] = 2;
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

int main() {
    {
        string st = rd();
        sscanf(st.c_str(), "%lld %lld %lld %lld", &N, &K, &ID, &MSGLEN);
    }
    m = (N + 1) / 2;
    HM = m * (m - 1);
    E = 2 * HM;
    initParams();
    G = (m + pTS - 1) / pTS;
    if (ID != 0) {
        if (pStatic)
            workerStatic();
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
