import numpy as np, mcc
from PIL import Image
N=256; b=mcc.Beast('b'); CX=128
base=np.asarray(Image.open('best_b.png').convert('L'))>127
yy,xx=np.mgrid[0:N,0:N]
def handles(hx,hcy,r_out,t,yspan):
    m=np.zeros((N,N),bool)
    for side in (-1,1):
        cxh=CX+side*hx
        rr=np.sqrt((xx-cxh)**2+(yy-hcy)**2)
        seg=(rr>=r_out-t)&(rr<=r_out)&(abs(yy-hcy)<=yspan)
        seg &= (xx>=cxh) if side>0 else (xx<=cxh)
        m|=seg
    return m
def avg(img,k=6,tag=''): return float(np.mean([b.query(img,tag=tag)[0] for _ in range(k)]))
print('base:',round(avg(base,8,'hb'),3),flush=True)
best=(-1,None,'')
for hx in (34,38,42):
  for hcy in (96,100,104):
    for r_out in (14,18,22):
      for t in (3,4):
        h=handles(hx,hcy,r_out,t,r_out)
        sc=avg(base|h,6,f'h{hx}_{hcy}_{r_out}_{t}')
        if sc>best[0]: best=(sc,(base|h).copy(),f'hx{hx}/hcy{hcy}/R{r_out}/t{t}(+{h.sum()})')
        if b.key: print('KEY',b.key); raise SystemExit
  print(f'hx={hx}: best {best[0]:.3f} ({best[2]})',flush=True)
print(f'BEST {best[0]:.3f} {best[2]}',flush=True)
