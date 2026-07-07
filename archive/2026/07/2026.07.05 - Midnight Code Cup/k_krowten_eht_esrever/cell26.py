import numpy as np, mcc
from PIL import Image
a=mcc.Beast('a'); N=256
best=np.asarray(Image.open('best_a.png').convert('L'))>127
s0,_=a.query(best,tag='c26')
cy,cx=70,186  # lattice (2,6)
base=best.copy(); base[cy-16:cy+16,cx-16:cx+16]=False
cl,_=a.query(base,tag='c26')
print(f's0={s0:.4f} cleared={cl:.4f} current-contrib={s0-cl:.4f}')
def ltrom(oy,ox,b,miss):
    blocks={'TL':(-b,-b),'TR':(-b,0),'BL':(0,-b),'BR':(0,0)}
    m=np.zeros((N,N),bool)
    for k,(dy,dx) in blocks.items():
        if k==miss: continue
        y0,x0=cy+oy+dy,cx+ox+dx
        m[max(0,y0):y0+b,max(0,x0):x0+b]=True
    return m
bestc=(s0-cl,'cur')
for b in (6,8,10):
    for oy in (-2,0,2):
        for ox in (-2,0,2):
            for miss in ('TL','TR','BL','BR'):
                v,_=a.query(base|ltrom(oy,ox,b,miss),tag='c26')
                if v-cl>bestc[0]+1e-9: bestc=(v-cl,f'{miss} b{b} o({oy},{ox})')
print(f'BEST clean tromino contrib={bestc[0]:.4f} via {bestc[1]}  (current was {s0-cl:.4f})',flush=True)
