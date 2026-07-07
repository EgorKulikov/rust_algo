import numpy as np, mcc
from PIL import Image
N=256; CX=128; b=mcc.Beast('b')
yy,xx=np.mgrid[0:N,0:N]
cur=np.asarray(Image.open('best_b.png').convert('L'))>127
def sym(m):  # mirror a right-side mask to both sides
    return m | m[:, ::-1]
def rect(y0,y1,x0,x1):
    m=np.zeros((N,N),bool); m[y0:y1, x0:x1]=True; return sym(m)
def arc(cy,cx,r,t):
    m=np.zeros((N,N),bool)
    for s in(-1,1):
        cxs=CX+s*(cx-CX); rr=np.sqrt((xx-cxs)**2+(yy-cy)**2)
        seg=(rr>=r-t)&(rr<=r); seg&=(xx>=cxs) if s>0 else (xx<=cxs); m|=seg
    return m
def compare(cand,k=12):
    d=[]
    for _ in range(k):
        s1=b.query(cur,tag='hc_b')[0]; s2=b.query(cand,tag='hc_c')[0]; d.append(s2-s1)
    d=np.array(d); return d.mean(), d.std()/np.sqrt(len(d))
cands={
 'top_bridge': cur|rect(76,82,162,178),
 'bot_bridge': cur|rect(104,110,150,178),
 'both_bridge': cur|rect(76,82,162,178)|rect(104,110,150,178),
 'extend_up': cur|arc(88,175,15,6),
 'extend_down': cur|arc(98,175,15,6),
 'thick_arc': cur|arc(92,175,15,8),
 'inner_arc': cur|arc(92,171,12,4),
 'bigger_r': cur|arc(92,176,17,6),
}
for name,c in cands.items():
    if np.array_equal(c,cur): continue
    m,se=compare(c,12); flag='*' if(m>3*se and m>0) else ' '
    print(f'{flag} {name}: {m:+.4f} +/- {se:.4f}',flush=True)
    if b.key: print('KEY',b.key); break
