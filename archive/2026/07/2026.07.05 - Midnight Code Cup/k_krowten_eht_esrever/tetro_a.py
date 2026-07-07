import numpy as np, mcc
from collections import deque
from PIL import Image
N=256; CELL=4; G=N//CELL
a=mcc.Beast('a')
def cm(x): return x.reshape(G,CELL,G,CELL).mean(axis=(1,3))>0.5
def toimg(c): return np.kron(c,np.ones((CELL,CELL),bool))
NB4=[(1,0),(-1,0),(0,1),(0,-1)]
def comps(grid,val):
    seen=np.zeros_like(grid); out=[]
    for i in range(G):
     for j in range(G):
      if grid[i,j]==val and not seen[i,j]:
        q=deque([(i,j)]);seen[i,j]=True;cell=[(i,j)]
        while q:
          y,x=q.popleft()
          for dy,dx in NB4:
            ny,nx=y+dy,x+dx
            if 0<=ny<G and 0<=nx<G and grid[ny,nx]==val and not seen[ny,nx]: seen[ny,nx]=True;q.append((ny,nx));cell.append((ny,nx))
        out.append(cell)
    return out
def groups_at(c,i,x,maxs=4):
    # all connected BLACK groups (size<=maxs) containing (i,x); c[i,x] must be black
    res=[]; frontier=[frozenset([(i,x)])]; seen=set(frontier)
    res.append((i,x)); allg=[frozenset([(i,x)])]
    cur={frozenset([(i,x)])}
    for _ in range(maxs-1):
        nxt=set()
        for gset in cur:
            for (y,xx) in gset:
                for dy,dx in NB4:
                    ny,nx=y+dy,xx+dx
                    if 0<=ny<G and 0<=nx<G and not c[ny,nx] and (ny,nx) not in gset:
                        ng=gset|{(ny,nx)}
                        if ng not in seen: seen.add(ng); nxt.add(ng); allg.append(ng)
        cur=nxt
    return allg
cur=np.asarray(Image.open('best_a.png').convert('L'))>127
cur_s,_=a.query(cur,tag='t_base'); print(f'base {cur_s:.4f}',flush=True)
rnd=0
while not a.key:
    rnd+=1; improved=0; c=cm(cur)
    # removal of white components
    for cell in comps(c,True):
        if a.key or len(cell)>10: continue
        c2=c.copy()
        for y,x in cell: c2[y,x]=False
        s,k=a.query(toimg(c2),tag='t_rem')
        if s>cur_s+1e-9: cur=toimg(c2); cur_s=s; c=c2; improved+=1
    # tetromino fills at black cells adjacent to white
    c=cm(cur)
    bd=[(i,x) for i in range(G) for x in range(G) if not c[i,x] and any(0<=i+dy<G and 0<=x+dx<G and c[i+dy,x+dx] for dy,dx in NB4)]
    for (i,x) in bd:
        if a.key: break
        if c[i,x]: continue
        bestmove=None; bestsc=cur_s
        for gset in groups_at(c,i,x,4):
            c2=c.copy()
            for (y,xx) in gset: c2[y,xx]=True
            s,k=a.query(toimg(c2),tag='t_add')
            if k: break
            if s>bestsc+1e-9: bestsc=s; bestmove=gset
        if bestmove:
            for (y,xx) in bestmove: c[y,xx]=True
            cur=toimg(c); cur_s=bestsc; improved+=1
    print(f'round {rnd}: score={cur_s:.4f} improved={improved}',flush=True)
    if improved==0: break
print(f'DONE best={cur_s:.4f} key={a.key}',flush=True)
