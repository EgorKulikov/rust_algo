#ifndef GREAT_DFS_LIVES_FOREVER
#define GREAT_DFS_LIVES_FOREVER

class Array {
    int a[10000001];

public:
    Array();

    int operator[](int x);

    void set(int x, int k);
};

void dfs(int, Array&, Array&);

int fa(int);
#endif

// write your code here
void dfs(int n, Array& f, Array& a) {
    for (int i = 1; i <= n; i++) {
        a.set(i, i);
    }
    for (int i = n; i > 1; i--) {
        int p = f[i];
        if (a[p] == p) {
            if (a[i] == i) {
                f.set(i, i);
                a.set(i, p);
                a.set(p, i);
                continue;
            }
            f.set(i, a[f[f[a[i]]]]);
            int c0 = a[i];
            int d0 = f[c0];
            int d1 = f[i];
            a.set(p, i);
            a.set(d0, p);
            f.set(i, d0);
            f.set(c0, p);
            continue;
        }
        if (a[i] == i) {
            f.set(i, i);
        } else {
            f.set(i, a[f[f[a[i]]]]);
        }
        int b0;
        if (a[a[f[p]]] == p) {
            b0 = a[f[p]];
        } else {
            b0 = f[a[p]];
        }
        int a0 = a[p];
        int a1 = a[a0];
        bool ch_a0 = a[a[f[a0]]] == a0;
        bool ch_a1 = a[a[f[a1]]] == a1;
        if (a[i] == i) {
            a.set(p, i);
            a.set(i, a0);
            if (ch_a1) {
                f.set(a1, i);
            }
            if (ch_a0) {
                f.set(a0, p);
            }
            f.set(i, b0);
            continue;
        }
        int c0 = a[i];
        int d0 = f[c0];
        int d1 = f[i];
        a.set(p, i);
        a.set(d0, a0);
        f.set(i, b0);
        f.set(c0, p);
        if (ch_a1) {
            f.set(a1, d0);
        }
        if (ch_a0) {
            f.set(a0, d1);
        }
    }
    for (int i = 1; i <= n; i++) {
        f.set(i, a[i]);
    }
    int at = 1;
    for (int i = 1; i <= n; i++) {
        a.set(i, at);
        at = f[at];
    }
}

// interactor
#ifndef ONLINE_JUDGE
#include <bits/stdc++.h>
using namespace std;
int n;

void quit(const string& s) {
    cerr << s << endl;
    exit(0);
}

Array::Array() {
    memset(a, 0, sizeof a);
}

int Array::operator[](int x) {
    if (x < 0 || x > n) quit("[" + to_string(x) + "]: Index out of range.");
    return a[x];
}

void Array::set(int x, int k) {
    if (x < 0 || x > n) quit("set(" + to_string(x) + "," + to_string(k) + "): Index out of range.");
    if (-1 > k || k > n) quit("set(" + to_string(x) + "," + to_string(k) + "): Value out of range.");
    a[x] = k;
}

Array f, a, fb;

int fa(int x) {
    return fb[x];
}

int main() {
    cin >> n;
    for (int i{2}; i <= n; ++i) {
        int x;
        cin >> x;
        f.set(i, x);
        fb.set(i, x);
    }
    dfs(n, f, a);
    for (int i{1}; i <= n; ++i) cout << a[i] << " ";
    return 0;
}
#endif
