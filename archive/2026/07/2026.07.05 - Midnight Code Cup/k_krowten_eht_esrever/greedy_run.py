import sys, mcc, greedy
name=sys.argv[1]; sched=tuple(int(x) for x in sys.argv[2].split(',')) if len(sys.argv)>2 else (8,4,2)
b=mcc.Beast(name)
print(f'[{name}] greedy seed best={b.best_score:.4f} sched={sched}',flush=True)
greedy.greedy(b, cell_schedule=sched, passes=4)
print(f'[{name}] done best={b.best_score:.4f} key={b.key}',flush=True)
