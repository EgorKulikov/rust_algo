# Fastest sequence to reach a 1-D position: DP over (distance covered, |pace|),
# paces change by at most one per step, enter and leave via pace 5.
def best_sequence(d):
    """List of |paces| (each moves 1 unit) with minimal total time, first and last 5."""
    if d == 0:
        return []
    INF = float('inf')
    dp = [[INF] * 6 for _ in range(d + 1)]
    prev = [[0] * 6 for _ in range(d + 1)]
    dp[1][5] = 5
    for i in range(1, d):
        for k in range(1, 6):
            if dp[i][k] == INF: continue
            for nk in (k - 1, k, k + 1):
                if 1 <= nk <= 5 and dp[i][k] + nk < dp[i + 1][nk]:
                    dp[i + 1][nk] = dp[i][k] + nk; prev[i + 1][nk] = k
    seq = []; k = 5; i = d
    assert dp[d][5] < INF
    while i >= 1:
        seq.append(k); k = prev[i][k]; i -= 1
    return seq[::-1]

def solve(inp):
    lines = inp.split('\n'); n = int(lines[0]); out = []
    for i in range(n):
        pos, limit = map(int, lines[1 + i].split())
        seq = best_sequence(abs(pos))
        if pos < 0: seq = [-p for p in seq]
        time = 2 + sum(abs(p) for p in seq)
        assert time <= limit, (pos, limit, time)
        out.append(' '.join(map(str, [0] + seq + [0])))
    return '\n'.join(out) + '\n'
