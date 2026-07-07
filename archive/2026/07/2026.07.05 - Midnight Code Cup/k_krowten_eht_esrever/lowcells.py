import numpy as np, mcc
from PIL import Image
a=mcc.Beast('a'); N=256
best=np.asarray(Image.open('best_a.png').convert('L'))>127
s0,_=a.query(best,tag='lc')
def test(cr,cc):
    cy,cx=cr*4+2,cc*4+2
    base=best.copy(); base[cy-16:cy+16,cx-16:cx+16]=False
    cl,_=a.query(base,tag='lc')
    def ltrom(oy,ox,b,miss):
        blocks={'TL':(-b,-b),'TR':(-b,0),'BL':(0,-b),'BR':(0,0)}
        m=np.zeros((N,N),bool)
        for k,(dy,dx) in blocks.items():
            if k==miss:continue
            y0,x0=cy+oy+dy,cx+ox+dx; m[max(0,y0):y0+b,max(0,x0):x0+b]=True
        return m
    bc=(s0-cl,'cur')
    for b in (6,8,10):
        for oy in (-2,0,2):
            for ox in (-2,0,2):
                for miss in ('TL','TR','BL','BR'):
                    v,_=a.query(base|ltrom(oy,ox,b,miss),tag='lc')
                    if v-cl>bc[0]+1e-9: bc=(v-cl,f'{miss}b{b}o({oy},{ox})')
    return cl, bc
for cr,cc in [(45,4),(52,4),(24,32)]:
    cl,bc=test(cr,cc)
    print(f'cell({cr},{cc}): current-contrib was low; BEST tromino contrib={bc[0]:.4f} via {bc[1]}',flush=True)
