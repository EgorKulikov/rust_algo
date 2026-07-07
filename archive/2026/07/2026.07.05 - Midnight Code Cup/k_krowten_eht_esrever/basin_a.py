import numpy as np, mcc
from PIL import Image
N=256; CELL=4; G=N//CELL
a=mcc.Beast('a')
best=np.asarray(Image.open('best_a.png').convert('L'))>127
best_s,_=a.query(best,tag='bh_seed'); print(f'seed {best_s:.4f}',flush=True)
rng=np.random.default_rng(0)
def cm(x): return x.reshape(G,CELL,G,CELL).mean(axis=(1,3))>0.5
def toimg(c): return np.kron(c,np.ones((CELL,CELL),bool))
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
def rand_move(c):
    mv=rng.integers(3); g=int(rng.integers(1,5))
    if mv==0: [c.__setitem__((y,x),True) for y,x in grow(c,False,g)]
    elif mv==1: [c.__setitem__((y,x),False) for y,x in grow(c,True,g)]
    else:
        [c.__setitem__((y,x),True) for y,x in grow(c,False,g)]
        [c.__setitem__((y,x),False) for y,x in grow(c,True,g)]
    return c
hop=0
while not a.key:
    hop+=1
    # perturb from best
    c=cm(best.copy()); K=int(rng.integers(1,3))
    for _ in range(K): c=rand_move(c)
    cur=toimg(c); cur_s,_=a.query(cur,tag=f'bh{hop}p')
    # greedy climb
    stall=0
    while stall<80 and not a.key:
        c2=cm(cur.copy()); c2=rand_move(c2); tr=toimg(c2)
        s,k=a.query(tr,tag=f'bh{hop}c')
        if s>cur_s: cur=tr; cur_s=s; stall=0
        else: stall+=1
    if cur_s>best_s:
        best=cur; best_s=cur_s; print(f'hop {hop}: NEW BEST {best_s:.4f} (perturb K={K})',flush=True)
    if hop%5==0: print(f'hop {hop}: best={best_s:.4f}',flush=True)
print(f'DONE best={best_s:.4f} key={a.key}',flush=True)
