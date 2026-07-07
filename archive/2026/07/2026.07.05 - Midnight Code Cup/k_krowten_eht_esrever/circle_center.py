import numpy as np, mcc
from PIL import Image
b=mcc.Beast('b'); N=256
yy,xx=np.mgrid[0:N,0:N]
rec=np.asarray(Image.open('best_b.png').convert('L'))>127
r0=np.sqrt((xx-128)**2+(yy-128)**2)
trophy=rec&(r0<66)
def avg(img,k=8,tag=''): return float(np.mean([b.query(img,tag=tag)[0] for _ in range(k)]))
best=(-1,'')
print('centered R118.5:',round(avg(np.abs(r0-118.5)<=2 | trophy if False else (np.abs(r0-118.5)<=2)|trophy,8,'cc_base'),3),flush=True)
for dy in (-2,-1,0,1,2):
  for dx in (0,1,2):  # x-symmetric, only test dx>=0
    r=np.sqrt((xx-(128+dx))**2+(yy-(128+dy))**2)
    img=(np.abs(r-118.5)<=2)|trophy
    sc=avg(img,6,f'cc_dx{dx}dy{dy}')
    if sc>best[0]: best=(sc,f'dx{dx}dy{dy}')
    if b.key: print('KEY',b.key); break
  print(f'dy={dy} row done, best {best[0]:.3f} ({best[1]})',flush=True)
# ellipse: vary y-radius
for ry in (116,117,118,119,120):
  r=np.sqrt((xx-128)**2+((yy-128)*118.5/ry)**2)
  img=(np.abs(r-118.5)<=2)|trophy
  print(f'ellipse ry={ry}: {avg(img,6,"cc_ell"):.3f}',flush=True)
print('BEST',round(best[0],3),best[1],flush=True)
