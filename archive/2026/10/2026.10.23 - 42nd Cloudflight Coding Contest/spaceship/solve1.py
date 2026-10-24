def solve(inp):
    lines = inp.split('\n'); n = int(lines[0])
    return '\n'.join(str(sum(map(int, lines[1 + i].split()))) for i in range(n)) + '\n'
