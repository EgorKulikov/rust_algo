import numpy as np, mcc
from PIL import Image
N=256; CX=128; b=mcc.Beast('b')
cur=np.asarray(Image.open('best_b.png').convert('L'))>127
def sym(m): return m|m[:,::-1]
def bottom_ext(y_start,x_start,slope,L,t):
    # from ear bottom (y_start,x_start) extend down with given slope (dx/dy), right side; mirror
    m=np.zeros((N,N),bool)
    for i in range(L):
        y=y_start+i; xc=x_start+slope*i
        for dx in range(-(t//2),t-(t//2)):
            xx=int(round(xc))+dx
            if 0<=y<N and 0<=xx<N: m[y,xx]=True
    return sym(m)
def compare(cand,k=12):
    d=[]
    for _ in range(k):
        s1=b.query(cur,tag='eb_b')[0]; s2=b.query(cand,tag='eb_c')[0]; d.append(s2-s1)
    d=np.array(d); return d.mean(), d.std()/np.sqrt(len(d))
# ear bottom is ~ (107, 177) on the right
for slope in (-0.6,-0.3,0.0,0.3):
  for L in (6,10,14):
    cand=cur|bottom_ext(107,177,slope,L,4)
    if np.array_equal(cand,cur): continue
    m,se=compare(cand,12); flag='*' if(m>3*se and m>0) else ' '
    print(f'{flag} slope={slope} L={L}: {m:+.4f} +/- {se:.4f}',flush=True)
    if b.key: print('KEY',b.key); break
