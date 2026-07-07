import numpy as np, mcc
from PIL import Image
a=mcc.Beast('a'); N=256
best=np.asarray(Image.open('best_a.png').convert('L'))>127
# polyomino shapes (in 4px-cell units, offsets)
SHAPES={
 'I2':[(0,0),(0,1)],'I3':[(0,0),(0,1),(0,2)],'L3':[(0,0),(1,0),(1,1)],
 'I4':[(0,0),(0,1),(0,2),(0,3)],'O4':[(0,0),(0,1),(1,0),(1,1)],'T4':[(0,0),(0,1),(0,2),(1,1)],
 'L4':[(0,0),(1,0),(2,0),(2,1)],'S4':[(0,1),(0,2),(1,0),(1,1)],
 'P5':[(0,0),(0,1),(1,0),(1,1),(2,0)],'X5':[(0,1),(1,0),(1,1),(1,2),(2,1)],
 'sq2':[(0,0),(0,1),(1,0),(1,1)],'sq3':[(i,j) for i in range(3) for j in range(3)],
}
def test_cell(cr,cc,cur_contrib):
    cy0,cx0=cr*4, cc*4  # cell-grid origin approx
    base=best.copy(); base[cr*4+2-14:cr*4+2+14, cc*4+2-14:cc*4+2+14]=False
    cl,_=a.query(base,tag='pc')
    bestv=(cl,'empty')
    # place shape at cell, trying a few pixel offsets
    for name,cells in SHAPES.items():
        h=max(y for y,x in cells)+1; w=max(x for y,x in cells)+1
        for py in (-8,-4,0,4):
            for px in (-8,-4,0,4):
                m=np.zeros((N,N),bool)
                for dy,dx in cells:
                    yy=cr*4+2+py+dy*4-2; xx=cc*4+2+px+dx*4-2
                    if 0<=yy<N-4 and 0<=xx<N-4: m[yy:yy+4,xx:xx+4]=True
                v,_=a.query(base|m,tag='pc')
                if v>bestv[0]: bestv=(v,f'{name}@({py},{px})')
    return cl, bestv[0], bestv[0]-cl, bestv[1], cur_contrib
for cr,cc,cur in [(24,32,0.864),(45,4,0.827),(52,4,0.827)]:
    cl,mx,d,desc,cc0=test_cell(cr,cc,cur)
    print(f'cell({cr},{cc}): best contrib={d:.3f} via {desc} (was {cc0} as square)',flush=True)
