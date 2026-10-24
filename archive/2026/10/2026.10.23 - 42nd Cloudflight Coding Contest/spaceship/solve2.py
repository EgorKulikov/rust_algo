def solve(inp):
    lines = inp.split('\n'); n = int(lines[0]); out = []
    for i in range(n):
        pos = time = 0
        for p in map(int, lines[1 + i].split()):
            if p == 0: time += 1
            elif p > 0: pos += 1; time += p
            else: pos -= 1; time += -p
        out.append(f'{pos} {time}')
    return '\n'.join(out) + '\n'
