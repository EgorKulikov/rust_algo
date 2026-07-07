import numpy as np, mcc
from collections import deque
from PIL import Image
N=256; CELL=4; G=N//CELL
a=mcc.Beast('a')
def load(): return np.asarray(Image.open('best_a.png').convert('L'))>127
def cm(x): return x.reshape(G,CELL,G,CELL).mean(axis=(1,3))>0.5
def toimg(c): return np.kron(c,np.ones((CELL,CELL),bool))
def comps(grid,val):
    seen=np.zeros_like(grid); out=[]
    for i in range(G):
     for j in range(G):
      if grid[i,j]==val and not seen[i,j]:
        q=deque([(i,j)]);seen[i,j]=True;cell=[(i,j)]
        while q:
          y,x=q.popleft()
          for dy,dx in((1,0),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,1)):
            ny,nx=y+dy,x+dx
            if 0<=ny<G and 0<=nx<G and grid[ny,nx]==val and not seen[ny,nx]: seen[ny,nx]=True;q.append((ny,nx));cell.append((ny,nx))
        out.append(cell)
    return out
cur=load(); cur_s,_=a.query(cur,tag='sy_base'); print(f'base {cur_s:.4f}',flush=True)
rnd=0
while not a.key:
    rnd+=1; c=cm(cur); improved=0
    # PASS 1: try removing each white component (wrong pieces)
    wc=comps(c,True)
    for cell in wc:
        if a.key or len(cell)>12: continue
        c2=c.copy()
        for y,x in cell: c2[y,x]=False
        s,k=a.query(toimg(c2),tag='sy_rem')
        if s>cur_s+1e-9: c=c2; cur=toimg(c); cur_s=s; improved+=1
    # PASS 2: try adding small groups at black cells adjacent to white
    c=cm(cur); cand=[]
    for i in range(G):
     for x in range(G):
      if not c[i,x] and any(0<=i+dy<G and 0<=x+dx<G and c[i+dy,x+dx] for dy,dx in((1,0),(-1,0),(0,1),(0,-1))): cand.append((i,x))
    for (i,x) in cand:
        if a.key: break
        # try adding this cell plus each subset growing to size 2,3,4 greedily
        for extra in [[], ]:
            c2=c.copy(); c2[i,x]=True
            s,k=a.query(toimg(c2),tag='sy_add1')
            if s>cur_s+1e-9: c=c2; cur=toimg(c); cur_s=s; improved+=1; continue
            # add a 2nd adjacent black cell
            for dy,dx in((1,0),(-1,0),(0,1),(0,-1)):
                ny,nx=i+dy,x+dx
                if 0<=ny<G and 0<=nx<G and not c[ny,nx]:
                    c3=c.copy(); c3[i,x]=True; c3[ny,nx]=True
                    s,k=a.query(toimg(c3),tag='sy_add2')
                    if s>cur_s+1e-9: c=c3; cur=toimg(c); cur_s=s; improved+=1; break
    print(f'round {rnd}: score={cur_s:.4f} improved={improved}',flush=True)
    if improved==0: break
print(f'DONE best={cur_s:.4f} key={a.key}',flush=True)
