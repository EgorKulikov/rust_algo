import sim
def solve(inp):
    lines = [l for l in inp.split('\n') if l.strip()]; n = int(lines[0]); out = []
    for i in range(n):
        xy, limit = lines[1 + 2 * i].split(); gx, gy = map(int, xy.split(',')); limit = int(limit)
        ax, ay = map(int, lines[2 + 2 * i].split(','))
        best = sim.plan((gx, gy), [((ax, ay), (0, 0))], limit)
        assert best, (gx, gy, ax, ay, limit)
        out.append(' '.join(map(str, best[1])) + '\n' + ' '.join(map(str, best[2])))
    return '\n\n'.join(out) + '\n'
