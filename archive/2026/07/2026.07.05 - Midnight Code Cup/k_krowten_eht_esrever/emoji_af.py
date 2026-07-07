import glob,os,numpy as np
from PIL import Image
import mcc
N=mcc.N; b=mcc.Beast('b')
yy,xx=np.mgrid[0:N,0:N]; r=np.sqrt((xx-N/2)**2+(yy-N/2)**2)
CIRCLE=np.abs(r-118.5)<=2.0
def avg(img,k=5,tag=''): return float(np.mean([b.query(img,tag=tag)[0] for _ in range(k)]))
def glyph(path,mode):
    a=np.asarray(Image.open(path).convert('RGBA'))
    m=a[:,:,3]>40 if mode=='alpha' else ((a[:,:,3]>40)&(a[:,:,:3].mean(2)>90))
    ys,xs=np.where(m); return m[ys.min():ys.max()+1,xs.min():xs.max()+1]
def place(mask,size,cy,cx=128):
    im=Image.fromarray((mask*255).astype('uint8')); ratio=size/im.height
    im=im.resize((max(1,int(im.width*ratio)),size),Image.LANCZOS); m=np.asarray(im)>127
    out=np.zeros((N,N),bool); y0,x0=cy-m.shape[0]//2,cx-m.shape[1]//2
    ys,xs=np.where(m); Y=y0+ys; X=x0+xs; ok=(Y>=0)&(Y<N)&(X>=0)&(X<N); out[Y[ok],X[ok]]=True; return out
best=(-1,'')
for path in sorted(glob.glob(mcc.HERE+'/emoji/apple_*.png')+glob.glob(mcc.HERE+'/emoji/facebook_*.png')):
    name=os.path.basename(path)[:-4]
    for mode in ('alpha','lumin'):
        try: g=glyph(path,mode)
        except: continue
        for size in (78,90,102):
            for cy in (108,116):
                sc=avg(CIRCLE|place(g,size,cy),5,f'af_{name}_{mode}_{size}_{cy}')
                if sc>best[0]: best=(sc,f'{name}/{mode}/sz{size}/cy{cy}')
                if b.key: print('KEY',b.key); raise SystemExit
    print(f'{name}: best {best[0]:.3f} ({best[1]})',flush=True)
print('BEST',round(best[0],3),best[1],flush=True)
