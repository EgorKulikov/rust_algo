// MCC D "Datacenter Imprisonment" — baseline: agent 0 explores solo, others idle.
// Maze: N odd, rooms at (odd,odd) always empty, (even,even) always wall.
// Empty cells form a tree => an untested passage between two rooms already in
// the known component must be a wall (never query it).
// Agent 0 runs best-first search over rooms (priority = Manhattan distance of
// the new room to the goal), one query per turn, claims once goal is reached.
#include <bits/stdc++.h>
using namespace std;

static char buf[1 << 16];

static void say(const string& s) {
    fputs(s.c_str(), stdout);
    fputc('\n', stdout);
    fflush(stdout);
}

static string readline_() {
    if (!fgets(buf, sizeof buf, stdin)) exit(0);
    string s(buf);
    while (!s.empty() && (s.back() == '\n' || s.back() == '\r')) s.pop_back();
    return s;
}

int main() {
    long long N, K, ID, MSG;
    {
        string st = readline_();
        sscanf(st.c_str(), "%lld %lld %lld %lld", &N, &K, &ID, &MSG);
    }
    if (ID != 0) {
        // bow out; merlin tolerates any number of halts as long as one agent stays
        say("halt");
        return 0;
    }
    if (N == 1) {
        say("!");
        return 0;
    }
    long long m = (N + 1) / 2;  // rooms per side; room (i,j) at cell (2i+1, 2j+1)
    auto rid = [&](long long i, long long j) { return i * m + j; };
    vector<char> vis(m * m, 0);
    vector<int> par(m * m, -1);  // parent room in discovery tree
    // frontier: (priority, from_room, dir) ; dirs: 0=U,1=L,2=D,3=R in room space
    const int DI[4] = {-1, 0, 1, 0}, DJ[4] = {0, -1, 0, 1};
    const char DC[4] = {'U', 'L', 'D', 'R'};
    priority_queue<tuple<long long, int, int>, vector<tuple<long long, int, int>>,
                   greater<>> pq;
    auto push_room = [&](long long i, long long j) {
        for (int d = 0; d < 4; d++) {
            long long ni = i + DI[d], nj = j + DJ[d];
            if (ni < 0 || nj < 0 || ni >= m || nj >= m) continue;
            if (vis[rid(ni, nj)]) continue;
            long long pri = (m - 1 - ni) + (m - 1 - nj);  // dist of NEW room to goal
            pq.emplace(pri, (int)rid(i, j), d);
        }
    };
    vis[rid(0, 0)] = 1;
    push_room(0, 0);
    int goal = (int)rid(m - 1, m - 1);
    while (!vis[goal]) {
        auto [pri, from, d] = pq.top();
        pq.pop();
        long long fi = from / m, fj = from % m;
        long long ni = fi + DI[d], nj = fj + DJ[d];
        if (vis[rid(ni, nj)]) continue;  // already connected => wall for sure
        // wall cell between rooms: ((2fi+1 + 2ni+1)/2, ...) = (fi+ni+1, fj+nj+1)
        char q[64];
        snprintf(q, sizeof q, "? %lld %lld", fi + ni + 1, fj + nj + 1);
        say(q);
        string rep = readline_();
        if (rep[0] == '1') {
            int to = (int)rid(ni, nj);
            vis[to] = 1;
            par[to] = from * 4 + d;
            push_room(ni, nj);
        }
    }
    // reconstruct path goal -> start, each room step = 2 grid moves
    string path;
    int cur = goal;
    while (cur != rid(0, 0)) {
        int from = par[cur] / 4, d = par[cur] % 4;
        path += DC[d];
        path += DC[d];
        cur = from;
    }
    reverse(path.begin(), path.end());
    say("! " + path);
    return 0;
}
