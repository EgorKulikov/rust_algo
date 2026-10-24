#!/usr/bin/env python3
"""Spaceship training helper: run.py <level> [solve|submit|all]"""
import sys, os, json, zipfile, importlib
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
import ccc
SLUG = 'training-ccc-2025.11-classic-spaceship-17252-d614b7cb'
HERE = os.path.dirname(os.path.abspath(__file__))

def level_dir(level):
    d = os.path.join(HERE, f'level{level}')
    if not os.path.isdir(d):
        g = ccc.Game(SLUG)
        z = g.level_files(level)
        os.makedirs(d); zipfile.ZipFile(z).extractall(d)
    return d

def solve(level):
    d = level_dir(level)
    mod = importlib.import_module(f'solve{level}')
    outs = {}
    for fid in ('0-example', '1-small', '2-large'):
        inp = open(os.path.join(d, f'in_level-{level}_{fid}.txt')).read()
        out = mod.solve(inp)
        open(os.path.join(d, f'my_out_{fid}.txt'), 'w').write(out)
        outs[fid] = out
    exp = open(os.path.join(d, f'out_level-{level}_0-example.txt')).read()
    ok = exp.split() == outs['0-example'].split()
    print(f'level {level}: example {"OK" if ok else "MISMATCH"}')
    if not ok:
        print('expected:', exp[:300]); print('got:', outs['0-example'][:300])
    return ok

def submit(level, fids=('1-small', '2-large')):
    g = ccc.Game(SLUG)
    d = level_dir(level)
    for fid in fids:
        code, res = g.submit(level, fid, os.path.join(d, f'my_out_{fid}.txt'))
        ev = res.get('evaluation', {}) if isinstance(res, dict) else {}
        cases = ev.get('cases', [])
        bad = [c for c in cases if not c.get('isCorrect')]
        print(f'submit {level} {fid}: http {code} correct={ev.get("isCorrect")} cases={len(cases)} wrong={len(bad)} cooldown={res.get("cooldownSec") if isinstance(res, dict) else res}')
        if bad: print('  first wrong cases:', bad[:3])
    print('score:', json.dumps(g.info().get('score', {}).get('gameScore')))

if __name__ == '__main__':
    level = int(sys.argv[1]); what = sys.argv[2] if len(sys.argv) > 2 else 'solve'
    sys.path.insert(0, HERE)
    if what in ('solve', 'all'):
        ok = solve(level)
    if what == 'submit' or (what == 'all' and ok):
        submit(level)
