import numpy as np, mcc
from PIL import Image
a = mcc.Beast('a'); N = 256; G = 64
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
cs, _ = a.query(cur, tag='bs_seed'); print(f'seed {cs:.4f}', flush=True)
for it in range(3):
    imp = 0
    for bi in range(G):
        for bj in range(G):
            t = cur.copy(); t[bi * 4:bi * 4 + 4, bj * 4:bj * 4 + 4] ^= True
            v, _ = a.query(t, tag='bs')
            if v > cs + 1e-9:
                cur = t; cs = v; imp += 1
        if a.key:
            break
    print(f'pass{it}: {cs:.4f} imp={imp}', flush=True)
    if imp == 0:
        break
print(f'DONE {cs:.4f} key={a.key}', flush=True)
