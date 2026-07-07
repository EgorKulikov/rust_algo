import numpy as np, mcc
from PIL import Image
N=256; b=mcc.Beast('b'); CX=128
base=np.asarray(Image.open('best_b.png').convert('L'))>127
yy,xx=np.mgrid[0:N,0:N]
def ears(cy,cx,r,t,fill=False):
    m=np.zeros((N,N),bool)
    for side in (-1,1):
        cxs=CX+side*(cx-CX)
        rr=np.sqrt((xx-cxs)**2+(yy-cy)**2)
        seg=(rr<=r) if fill else ((rr>=r-t)&(rr<=r))
        seg &= (xx>=cxs) if side>0 else (xx<=cxs)
        m|=seg
    return m
def avg(img,k=8,tag=''): return float(np.mean([b.query(img,tag=tag)[0] for _ in range(k)]))
print('base:',round(avg(base,10,'a3b'),3),flush=True)
best=(-1,None,'')
for cx in (173,176,179):
  for cy in (86,90,94):
    for r in (14,17,20,24):
      for t in (5,7,10):
        h=ears(cy,cx,r,t)
        sc=avg(base|h,8,f'a3_{cx}_{cy}_{r}_{t}')
        if sc>best[0]: best=(sc,(base|h).copy(),f'cx{cx}/cy{cy}/r{r}/t{t}(+{h.sum()})'); print(f'  NEW {sc:.3f} {best[2]}',flush=True)
        if b.key: print('KEY',b.key); raise SystemExit
  print(f'cx={cx} done, best {best[0]:.3f}',flush=True)
print(f'BEST {best[0]:.3f} {best[2]}',flush=True)
