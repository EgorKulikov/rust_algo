import numpy as np, mcc
from PIL import Image
N=256; CELL=4; G=N//CELL
a=mcc.Beast('a')
TET=[[(0,0),(0,1),(0,2),(0,3)],[(0,0),(1,0),(2,0),(3,0)],[(0,0),(0,1),(1,0),(1,1)],
 [(0,0),(0,1),(0,2),(1,1)],[(0,1),(1,0),(1,1),(2,1)],[(1,0),(1,1),(1,2),(0,1)],[(0,0),(1,0),(1,1),(2,0)],
 [(0,1),(0,2),(1,0),(1,1)],[(0,0),(1,0),(1,1),(2,1)],[(0,0),(0,1),(1,1),(1,2)],[(0,1),(1,0),(1,1),(2,0)],
 [(0,0),(1,0),(2,0),(2,1)],[(0,0),(0,1),(0,2),(1,0)],[(0,0),(0,1),(1,1),(2,1)],[(0,2),(1,0),(1,1),(1,2)],
 [(0,1),(1,1),(2,0),(2,1)],[(0,0),(1,0),(1,1),(1,2)],[(0,0),(0,1),(1,0),(2,0)],[(0,0),(0,1),(0,2),(1,2)]]
def cm(x): return x.reshape(G,CELL,G,CELL).mean(axis=(1,3))>0.5
def toimg(c): return np.kron(c,np.ones((CELL,CELL),bool))
best=np.asarray(Image.open('best_a.png').convert('L'))>127
best_s,_=a.query(best,tag='lns_seed'); print(f'seed {best_s:.4f}',flush=True)
rng=np.random.default_rng(0)
it=0
while not a.key:
    it+=1
    c=cm(best.copy())
    # RUIN: clear a random region (size R x R cells)
    R=int(rng.integers(8,20)); y0=int(rng.integers(0,G-R)); x0=int(rng.integers(0,G-R))
    c[y0:y0+R, x0:x0+R]=False
    cur=toimg(c); cur_s,_=a.query(cur,tag=f'lns{it}r')
    # RECREATE: greedily add tetrominoes/cells within the region until stall
    stall=0
    while stall<60 and not a.key:
        # candidate gap cells inside region adjacent to any white
        gy=int(rng.integers(y0,y0+R)); gx=int(rng.integers(x0,x0+R))
        shp=TET[rng.integers(len(TET))]
        cells=[(gy+dy,gx+dx) for dy,dx in shp]
        if any(not(0<=y<G and 0<=x<G) or c[y,x] for y,x in cells): stall+=1; continue
        c2=c.copy()
        for y,x in cells: c2[y,x]=True
        s,k=a.query(toimg(c2),tag=f'lns{it}c')
        if s>cur_s: c=c2; cur=toimg(c); cur_s=s; stall=0
        else: stall+=1
    if cur_s>best_s:
        best=cur; best_s=cur_s; print(f'it {it}: NEW BEST {best_s:.4f} (R={R})',flush=True)
    if it%10==0: print(f'it {it}: best={best_s:.4f}',flush=True)
print(f'DONE best={best_s:.4f}',flush=True)
