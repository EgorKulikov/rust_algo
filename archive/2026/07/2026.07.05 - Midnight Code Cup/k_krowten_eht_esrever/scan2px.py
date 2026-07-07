import numpy as np, mcc
from PIL import Image
a=mcc.Beast('a'); N=256
cur=np.asarray(Image.open('best_a.png').convert('L'))>127
cs,_=a.query(cur,tag='s2_seed'); print(f'seed {cs:.4f}',flush=True)
# only scan 2px blocks near existing white (edges) — where non-square refinement helps
for it in range(2):
    imp=0
    white=np.argwhere(cur)
    # candidate 2px block top-lefts near white (dilate region)
    cand=set()
    for y,x in white:
        for dy in range(-4,5,2):
            for dx in range(-4,5,2):
                by,bx=((y+dy)//2)*2,((x+dx)//2)*2
                if 0<=by<N-1 and 0<=bx<N-1: cand.add((by,bx))
    for by,bx in sorted(cand):
        t=cur.copy(); t[by:by+2,bx:bx+2]^=True
        v,_=a.query(t,tag='s2')
        if v>cs+1e-9: cur=t; cs=v; imp+=1
        if a.key: break
    print(f'pass{it}: {cs:.4f} imp={imp} cand={len(cand)}',flush=True)
    if imp==0: break
    Image.fromarray((cur*255).astype('uint8')).save('best_a.png')
print(f'DONE {cs:.4f} key={a.key}',flush=True)
