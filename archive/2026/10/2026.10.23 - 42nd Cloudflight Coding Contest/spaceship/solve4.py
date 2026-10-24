from solve3 import best_sequence

def axis(v):
    seq = best_sequence(abs(v))
    if v < 0: seq = [-p for p in seq]
    return [0] + seq + [0]

def solve(inp):
    lines = inp.split('\n'); n = int(lines[0]); out = []
    for i in range(n):
        xy, limit = lines[1 + i].split(); x, y = map(int, xy.split(',')); limit = int(limit)
        sx, sy = axis(x), axis(y)
        assert max(sum(map(abs, sx)) + 2, sum(map(abs, sy)) + 2) <= limit, (x, y, limit)
        out.append(' '.join(map(str, sx)) + '\n' + ' '.join(map(str, sy)))
    return '\n\n'.join(out) + '\n'
