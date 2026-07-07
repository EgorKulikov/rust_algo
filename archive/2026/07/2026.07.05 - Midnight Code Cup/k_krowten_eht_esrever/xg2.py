import numpy as np, mcc
from PIL import Image
a=mcc.Beast('a'); N=256
best=np.asarray(Image.open('best_a.png').convert('L'))>127
def xg(cr,cc,sizes):
    cy,cx=cr*4+2,cc*4+2
    base=best.copy(); base[cy-16:cy+16,cx-16:cx+16]=False
    cl,_=a.query(base,tag='xg2')
    cand=[]
    for s in sizes:
        for oy in range(-16,16,s):
            for ox in range(-16,16,s):
                m=np.zeros((N,N),bool); m[cy+oy:cy+oy+s,cx+ox:cx+ox+s]=True; cand.append(m)
    cell=np.zeros((N,N),bool); cv=cl
    for _ in range(8):
        bg=(cv,None)
        for m in cand:
            v,_=a.query(base|(cell^m),tag='xg2')
            if v>bg[0]+1e-9: bg=(v,cell^m)
        if bg[1] is None: break
        cell,cv=bg[1],bg[0]
    return cl,cv,cv-cl
for cr,cc in [(24,32),(45,53)]:
    cl,cv,d=xg(cr,cc,(8,4))
    print(f'cell({cr},{cc}): XOR total={cv:.4f} contrib={d:.4f}',flush=True)
