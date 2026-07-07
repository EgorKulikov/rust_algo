import numpy as np, mcc
from PIL import Image
N=256; CELL=4; G=N//CELL
a=mcc.Beast('a')
cur=np.asarray(Image.open('best_a.png').convert('L'))>127
cur_s,_=a.query(cur,tag='j_base')
print(f'base {cur_s:.4f}',flush=True)
import sys; rng=np.random.default_rng(int(sys.argv[1]) if len(sys.argv)>1 else 0)
def cellmask(cur):  # 64x64 bool of cell states (majority)
    return cur.reshape(G,CELL,G,CELL).mean(axis=(1,3))>0.5
def grow(cm, want, g):
    # pick random cell with state 'want', grow connected group of size g of same state
    ys,xs=np.where(cm==want)
    if len(ys)==0: return []
    k=rng.integers(len(ys)); grp=[(ys[k],xs[k])]; s=set(grp)
    while len(grp)<g:
        y,x=grp[rng.integers(len(grp))]
        nb=[(y+dy,x+dx) for dy,dx in((1,0),(-1,0),(0,1),(0,-1)) if 0<=y+dy<G and 0<=x+dx<G]
        rng.shuffle(nb); added=False
        for ny,nx in nb:
            if cm[ny,nx]==want and (ny,nx) not in s: grp.append((ny,nx)); s.add((ny,nx)); added=True; break
        if not added: break
    return grp
best=cur_s; improved=0
for it in range(20000):
    if a.key: break
    cm=cellmask(cur)
    move=rng.integers(3)  # 0 add group, 1 remove group, 2 swap
    g=int(rng.integers(1,6))
    trial=cur.copy()
    if move==0:  grp=grow(cm,False,g); val=True
    elif move==1: grp=grow(cm,True,g); val=False
    else:
        grp=grow(cm,False,g); [trial.__setitem__((slice(y*CELL,y*CELL+CELL),slice(x*CELL,x*CELL+CELL)),True) for y,x in grp]
        grp2=grow(cm,True,g); [trial.__setitem__((slice(y*CELL,y*CELL+CELL),slice(x*CELL,x*CELL+CELL)),False) for y,x in grp2]; grp=[]
    for y,x in grp: trial[y*CELL:y*CELL+CELL,x*CELL:x*CELL+CELL]=val
    s,k=a.query(trial,tag=f'j{it}')
    if s>cur_s: cur=trial; cur_s=s; improved+=1; print(f'it={it} improved -> {s:.4f} (move{move} g{g})',flush=True)
    if it%200==0: print(f'it={it} cur={cur_s:.4f} improved={improved}',flush=True)
print(f'DONE best={cur_s:.4f} improved={improved}',flush=True)
