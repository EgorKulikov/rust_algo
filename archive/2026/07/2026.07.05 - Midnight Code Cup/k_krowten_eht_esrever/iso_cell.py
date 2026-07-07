import numpy as np, mcc
a=mcc.Beast('a'); N=256
cy,cx=24*4+2,25*4+2
# isolated greedy: maximize solo score of cell content via 2px blocks
cur=np.zeros((N,N),bool); cur[cy-6:cy+6,cx-6:cx+6]=True  # seed 12px square
cs,_=a.query(cur,tag='ic'); print('seed square iso:',round(cs,4),flush=True)
for it in range(4):
    imp=0
    for by in range(cy-16,cy+16,2):
        for bx in range(cx-16,cx+16,2):
            t=cur.copy(); t[by:by+2,bx:bx+2]^=True
            s,_=a.query(t,tag='ic')
            if s>cs+1e-9: cur=t; cs=s; imp+=1
    print(f'iso pass{it}: {cs:.4f} imp={imp}',flush=True)
    if imp==0: break
print('MAX isolated cell score:',round(cs,4),'(square plateau was 0.1235)',flush=True)
