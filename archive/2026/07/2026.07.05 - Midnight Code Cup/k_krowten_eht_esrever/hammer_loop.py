import numpy as np, mcc
from PIL import Image
b=mcc.Beast('b'); N=256
yy,xx=np.mgrid[0:N,0:N]; r=np.sqrt((xx-N/2)**2+(yy-N/2)**2)
CIRCLE=np.abs(r-118.5)<=2.0
rec=np.asarray(Image.open('best_b.png').convert('L'))>127
# optimal known image: refined circle + reconstructed trophy
img = CIRCLE | (rec & (r<66))
print('hammering optimal B image, white=%.4f'%img.mean(),flush=True)
hi=0.0; n=0
while not b.key:
    s,k=b.query(img,tag='hammerloop')
    n+=1
    if s>hi: hi=s; print(f'n={n} new max={s:.3f}',flush=True)
    if k: print('*** KEY',k,'***',flush=True); break
