import numpy as np, mcc
from PIL import Image
N=256; b=mcc.Beast('b')
cur=np.asarray(Image.open('best_b.png').convert('L'))>127
yy,xx=np.mgrid[0:N,0:N]; r=np.sqrt((xx-128)**2+(yy-128)**2)
oldcirc=np.abs(r-118.5)<=2
interior=cur&~oldcirc  # trophy without the circle
def mk(R,t): return interior|(np.abs(r-R)<=t/2.0)
def cmp(cand,k=16):
    d=[b.query(cand,tag='cfc')[0]-b.query(cur,tag='cfb')[0] for _ in range(k)]
    d=np.array(d); return d.mean(),d.std()/np.sqrt(len(d))
best=(0,None,'')
for R in (117.5,118.0,118.5,119.0,119.5):
  for t in (3,4,5):
    c=mk(R,t)
    if np.array_equal(c,cur): print(f'R{R}t{t} = current'); continue
    m,se=cmp(c,16); flag='*' if(m>3*se and m>0) else ' '
    print(f'{flag} R={R} t={t}: {m:+.4f} +/- {se:.4f}',flush=True)
    if m>best[0] and m>3*se: best=(m,c,f'R{R}t{t}')
    if b.key: print('KEY',b.key);break
if best[1] is not None:
    Image.fromarray((best[1]*255).astype('uint8')).save('best_b.png'); print('SAVED',best[2],'+%.3f'%best[0],flush=True)
else: print('circle already optimal',flush=True)
