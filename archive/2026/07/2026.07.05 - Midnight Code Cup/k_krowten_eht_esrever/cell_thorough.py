import numpy as np, mcc
from PIL import Image
a=mcc.Beast('a'); N=256
best=np.asarray(Image.open('best_a.png').convert('L'))>127
def contrib_test(cr,cc):
    cy,cx=cr*4+2,cc*4+2
    base=best.copy(); base[cy-16:cy+16,cx-16:cx+16]=False
    cleared,_=a.query(base,tag='ct')
    # try many seeds, pixel-refine best
    def score(cell): return a.query(base|cell,tag='ct')[0]
    def sq(oy,ox,s):
        m=np.zeros((N,N),bool); m[cy+oy-s//2:cy+oy+s-s//2, cx+ox-s//2:cx+ox+s-s//2]=True; return m
    seeds=[np.zeros((N,N),bool)]
    for oy in (-4,0,4):
        for ox in (-4,0,4):
            for s in (6,10,14,18): seeds.append(sq(oy,ox,s))
    best_cell=(None,-1)
    for sd in seeds:
        v=score(sd)
        if v>best_cell[1]: best_cell=(sd.copy(),v)
    # pixel-refine best seed (2px blocks)
    cell=best_cell[0]; cv=best_cell[1]
    for it in range(2):
        imp=0
        for by in range(cy-16,cy+16,2):
            for bx in range(cx-16,cx+16,2):
                t=cell.copy(); t[by:by+2,bx:bx+2]^=True
                v=score(t)
                if v>cv+1e-9: cell=t; cv=v; imp+=1
        if imp==0: break
    return cleared, cv, cv-cleared
for cell in [(24,25),(45,53),(31,60)]:
    cl,mx,d=contrib_test(*cell)
    print(f'cell {cell}: cleared={cl:.3f} best_total={mx:.4f} contribution={d:.4f}',flush=True)
