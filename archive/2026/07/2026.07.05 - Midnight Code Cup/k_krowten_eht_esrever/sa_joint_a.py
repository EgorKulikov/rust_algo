import numpy as np, math, mcc
from PIL import Image
N=256; CELL=4; G=N//CELL
a=mcc.Beast('a')
cur=np.asarray(Image.open('best_a.png').convert('L'))>127
cur_s,_=a.query(cur,tag='sj_base'); best=cur.copy(); best_s=cur_s
print(f'base {cur_s:.4f}',flush=True)
rng=np.random.default_rng(7)
def cm(x): return x.reshape(G,CELL,G,CELL).mean(axis=(1,3))>0.5
def grow(c,want,g):
    ys,xs=np.where(c==want)
    if len(ys)==0: return []
    k=rng.integers(len(ys)); grp=[(ys[k],xs[k])]; s=set(grp)
    while len(grp)<g:
        y,x=grp[rng.integers(len(grp))]
        nb=[(y+dy,x+dx) for dy,dx in((1,0),(-1,0),(0,1),(0,-1)) if 0<=y+dy<G and 0<=x+dx<G and c[y+dy,x+dx]==want and (y+dy,x+dx) not in s]
        if not nb: break
        ny,nx=nb[rng.integers(len(nb))]; grp.append((ny,nx)); s.add((ny,nx))
    return grp
T0,T1,ITER=0.15,0.008,40000; since=0
for it in range(ITER):
    if a.key: break
    T=T0*(T1/T0)**(it/ITER)
    c=cm(cur); mv=rng.integers(3); g=int(rng.integers(1,5)); tr=cur.copy()
    if mv==0:
        for y,x in grow(c,False,g): tr[y*CELL:y*CELL+CELL,x*CELL:x*CELL+CELL]=True
    elif mv==1:
        for y,x in grow(c,True,g): tr[y*CELL:y*CELL+CELL,x*CELL:x*CELL+CELL]=False
    else:
        for y,x in grow(c,False,g): tr[y*CELL:y*CELL+CELL,x*CELL:x*CELL+CELL]=True
        for y,x in grow(c,True,g): tr[y*CELL:y*CELL+CELL,x*CELL:x*CELL+CELL]=False
    s,k=a.query(tr,tag=f'sj{it}'); d=s-cur_s
    if d>=0 or rng.random()<math.exp(d/max(T,1e-6)): cur=tr; cur_s=s
    if s>best_s: best_s=s; best=tr.copy(); since=0; print(f'it={it} T={T:.3f} best={s:.4f} (mv{mv}g{g})',flush=True)
    else: since+=1
    if since>=800: cur=best.copy(); cur_s=best_s; since=0; T0=min(T0*1.3,4); print(f'reheat T0={T0:.2f}',flush=True)
    if it%500==0: print(f'it={it} cur={cur_s:.3f} best={best_s:.4f}',flush=True)
print(f'DONE best={best_s:.4f} key={a.key}',flush=True)
