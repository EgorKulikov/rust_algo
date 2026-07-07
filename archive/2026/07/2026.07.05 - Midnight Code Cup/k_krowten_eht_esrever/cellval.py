import numpy as np, mcc
from PIL import Image
a=mcc.Beast('a'); N=256
best=np.asarray(Image.open('best_a.png').convert('L'))>127
rows=[r*4+2 for r in range(3,64,7)]; cols=[c*4+2 for c in range(4,64,7)]
s0,_=a.query(best,tag='cv'); 
res=[]
for cr in range(3,64,7):
    for cc in range(4,64,7):
        cy,cx=cr*4+2,cc*4+2
        t=best.copy(); t[cy-14:cy+14,cx-14:cx+14]=False
        s,_=a.query(t,tag='cv')
        ncells=int(best[cy-14:cy+14,cx-14:cx+14].sum())//16  # ~4px-cells of content
        res.append((s0-s, ncells, cr, cc))
res.sort(reverse=True)
print(f's0={s0:.3f}')
print('TOP value cells (contrib, ~4px-cells content):')
for d,n,cr,cc in res[:12]: print(f'  contrib={d:.3f} content={n}cells at ({cr},{cc})')
print('BOTTOM value cells:')
for d,n,cr,cc in res[-12:]: print(f'  contrib={d:.3f} content={n}cells at ({cr},{cc})')
import numpy as np
arr=np.array([(d,n) for d,n,_,_ in res])
# correlation contribution vs content size
print(f'corr(contrib, content-size) = {np.corrcoef(arr[:,0],arr[:,1])[0,1]:.3f}')
# group by size
for lo,hi,lbl in [(1,3,'tiny 1-3'),(4,6,'small 4-6'),(7,12,'med 7-12'),(13,99,'big 13+')]:
    m=(arr[:,1]>=lo)&(arr[:,1]<=hi)
    if m.sum(): print(f'  {lbl}cells: n={int(m.sum())} mean-contrib={arr[m,0].mean():.3f}')
