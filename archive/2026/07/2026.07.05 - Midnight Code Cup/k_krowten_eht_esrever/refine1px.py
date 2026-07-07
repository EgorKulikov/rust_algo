import mcc, boundary
b=mcc.Beast('a')
print(f'[A] 1px refine seed={b.best_score:.4f}',flush=True)
boundary.refine(b, cell_schedule=(1,), margin=4, passes=6)
print(f'[A] done best={b.best_score:.4f} key={b.key}',flush=True)
