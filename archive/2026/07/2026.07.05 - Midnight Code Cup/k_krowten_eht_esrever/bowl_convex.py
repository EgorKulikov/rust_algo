import numpy as np, mcc
from PIL import Image
N = 256; CX = 128; b = mcc.Beast('b')
cur = np.asarray(Image.open('best_b.png').convert('L')) > 127
Y0, Y1 = 84, 123


def bowl(hw0, p):
    img = cur.copy(); img[Y0:Y1, 90:167] = False
    for y in range(Y0, Y1):
        u = (y - Y0) / (Y1 - 1 - Y0)
        hw = hw0 * (1 - u ** p)
        if hw < 3:
            continue
        img[y, int(round(CX - hw)):int(round(CX + hw)) + 1] = True
    return img


def cmp(cand, k=14):
    d = [b.query(cand, tag='vc')[0] - b.query(cur, tag='vb')[0] for _ in range(k)]
    d = np.array(d); return d.mean(), d.std() / np.sqrt(len(d))


best = (0, None, '')
for hw0 in (30, 31.5, 33):
    for p in (0.7, 0.85, 1.0, 1.3):
        c = bowl(hw0, p)
        m, se = cmp(c, 14); flag = '*' if (m > 3 * se and m > 0) else ' '
        print(f'{flag} hw0={hw0} p={p}: {m:+.4f} +/- {se:.4f}', flush=True)
        if m > best[0] and m > 3 * se:
            best = (m, c, f'hw0{hw0}p{p}')
        if b.key:
            print('KEY', b.key); break
if best[1] is not None:
    Image.fromarray((best[1] * 255).astype('uint8')).save('best_b.png')
    print('SAVED', best[2], '+%.3f' % best[0], flush=True)
else:
    print('no convex bowl improvement', flush=True)
