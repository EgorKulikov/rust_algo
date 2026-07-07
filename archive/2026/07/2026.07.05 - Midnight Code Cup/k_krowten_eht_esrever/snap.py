import numpy as np, mcc
from PIL import Image
a=mcc.Beast('a'); N=256
best=np.asarray(Image.open('best_a.png').convert('L'))>127
s0,_=a.query(best,tag='snap'); print('current best_a:',round(s0,4),flush=True)
clean=np.zeros((N,N),bool); sub={}
for ri in range(9):
    for ci in range(9):
        ty,tx=8+28*ri, 8+28*ci; s=[]
        for by in (0,8):
            for bx in (0,8):
                on=best[ty+by:ty+by+8, tx+bx:tx+bx+8].mean()>0.5
                s.append(int(on))
                if on: clean[ty+by:ty+by+8, tx+bx:tx+bx+8]=True
        sub[(ri,ci)]=tuple(s)
sc,_=a.query(clean,tag='snap'); print('CLEAN SNAP:',round(sc,4),'delta',round(sc-s0,4),flush=True)
if sc>s0:
    Image.fromarray((clean*255).astype('uint8')).save('best_a.png'); print('SAVED clean snap',flush=True)
np.save('subsets.npy', sub, allow_pickle=True)
