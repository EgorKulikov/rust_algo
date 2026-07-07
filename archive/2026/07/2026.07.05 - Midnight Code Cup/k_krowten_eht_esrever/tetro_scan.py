import numpy as np, mcc
from collections import deque
from PIL import Image
N=256; CELL=4; G=N//CELL
a=mcc.Beast('a')
TET=[[(0,0),(0,1),(0,2),(0,3)],[(0,0),(1,0),(2,0),(3,0)],
 [(0,0),(0,1),(1,0),(1,1)],
 [(0,0),(0,1),(0,2),(1,1)],[(0,1),(1,0),(1,1),(2,1)],[(1,0),(1,1),(1,2),(0,1)],[(0,0),(1,0),(1,1),(2,0)],
 [(0,1),(0,2),(1,0),(1,1)],[(0,0),(1,0),(1,1),(2,1)],
 [(0,0),(0,1),(1,1),(1,2)],[(0,1),(1,0),(1,1),(2,0)],
 [(0,0),(1,0),(2,0),(2,1)],[(0,0),(0,1),(0,2),(1,0)],[(0,0),(0,1),(1,1),(2,1)],[(0,2),(1,0),(1,1),(1,2)],
 [(0,1),(1,1),(2,0),(2,1)],[(0,0),(1,0),(1,1),(1,2)],[(0,0),(0,1),(1,0),(2,0)],[(0,0),(0,1),(0,2),(1,2)]]
def cm(x): return x.reshape(G,CELL,G,CELL).mean(axis=(1,3))>0.5
def toimg(c): return np.kron(c,np.ones((CELL,CELL),bool))
def comps(grid):
    seen=np.zeros_like(grid); out=[]
    for i in range(G):
     for j in range(G):
      if grid[i,j] and not seen[i,j]:
        q=deque([(i,j)]);seen[i,j]=True;cell=[(i,j)]
        while q:
          y,x=q.popleft()
          for dy,dx in((1,0),(-1,0),(0,1),(0,-1)):
            ny,nx=y+dy,x+dx
            if 0<=ny<G and 0<=nx<G and grid[ny,nx] and not seen[ny,nx]: seen[ny,nx]=True;q.append((ny,nx));cell.append((ny,nx))
        out.append(cell)
    return out
cur=np.asarray(Image.open('best_a.png').convert('L'))>127
cur_s,_=a.query(cur,tag='ts_seed'); print(f'seed {cur_s:.4f}',flush=True)
rnd=0
while not a.key:
    rnd+=1; c=cm(cur); improved=0; tested=0
    # remove only tiny (<=2) white components (cheap)
    for cell in comps(c):
        if a.key or len(cell)>2: continue
        c2=c.copy()
        for y,x in cell: c2[y,x]=False
        s,k=a.query(toimg(c2),tag='ts_rem'); tested+=1
        if s>cur_s+1e-9: cur=toimg(c2); cur_s=s; c=c2; improved+=1
    # add each tetromino at each boundary gap (anchor at empty cell adjacent to white)
    c=cm(cur)
    gaps=[(i,j) for i in range(G) for j in range(G) if not c[i,j] and any(0<=i+dy<G and 0<=j+dx<G and c[i+dy,j+dx] for dy,dx in((1,0),(-1,0),(0,1),(0,-1)))]
    for (i,j) in gaps:
        if a.key: break
        for shp in TET:
            cells=[(i+dy,j+dx) for dy,dx in shp]
            if any(not(0<=y<G and 0<=x<G) or c[y,x] for y,x in cells): continue
            c2=c.copy()
            for y,x in cells: c2[y,x]=True
            s,k=a.query(toimg(c2),tag='ts_add'); tested+=1
            if s>cur_s+1e-9: cur=toimg(c2); cur_s=s; c=c2; improved+=1; break
    print(f'round {rnd}: score={cur_s:.4f} improved={improved} tested={tested}',flush=True)
    if improved==0: break
print(f'DONE best={cur_s:.4f} key={a.key}',flush=True)
