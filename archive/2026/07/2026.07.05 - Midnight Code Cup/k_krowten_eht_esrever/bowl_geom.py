import numpy as np, mcc
from PIL import Image
N=256; CX=128; b=mcc.Beast('b')
cur=np.asarray(Image.open('best_b.png').convert('L'))>127
def geombowl(y0,y1,hw0,slope):
    # replace bowl body rows y0..y1 with straight-edged trapezoid centered CX
    img=cur.copy()
    img[y0:y1,90:167]=False  # clear old body (keep rim above y0, keep handles outside x90-167? handles are x>167/<90 so safe)
    for y in range(y0,y1):
        hw=hw0-slope*(y-y0)
        if hw<3: continue
        img[y, int(round(CX-hw)):int(round(CX+hw))+1]=True
    return img
def cmp(cand,k=14):
    d=[b.query(cand,tag='gc')[0]-b.query(cur,tag='gb')[0] for _ in range(k)]
    d=np.array(d); return d.mean(),d.std()/np.sqrt(len(d))
# body rows ~84-122, halfwidth from ~31.5 down. search hw0 and slope
best=(0,None,'')
for hw0 in (30,31.5,33):
  for slope in (0.60,0.667,0.73):
    c=geombowl(84,123,hw0,slope)
    if np.array_equal(c,cur): continue
    m,se=cmp(c,14); flag='*' if(m>3*se and m>0) else ' '
    print(f'{flag} hw0={hw0} slope={slope}: {m:+.4f} +/- {se:.4f}',flush=True)
    if m>best[0] and m>3*se: best=(m,c,f'hw0{hw0}s{slope}')
    if b.key: print('KEY',b.key);break
if best[1] is not None:
    Image.fromarray((best[1]*255).astype('uint8')).save('best_b.png'); print('SAVED',best[2],'+%.3f'%best[0],flush=True)
else: print('no geom bowl improvement',flush=True)
