#!/usr/bin/env python3
"""Scout for mover clusters: hop along +x, at each spot take 3 frames and
count moving pixels."""
import sys
import time
import numpy as np
from PIL import Image
sys.path.insert(0, '..')
from nav import goto, view

y0 = -9862.099012438162
start = float(sys.argv[1])
end = float(sys.argv[2])
step = float(sys.argv[3])

x = start
while (step > 0 and x <= end) or (step < 0 and x >= end):
    s = goto(x, y0, tol=60)
    px = s['position']['x']
    imgs = []
    for i in range(3):
        view('scout_tmp.png')
        imgs.append(np.array(Image.open('scout_tmp.png'), dtype=np.int16))
        time.sleep(1.5)
    d1 = (np.abs(imgs[1] - imgs[0]) > 30).sum()
    d2 = (np.abs(imgs[2] - imgs[1]) > 30).sum()
    moving = min(d1, d2)
    print(f'x={px:9.1f} moving_px={moving}', flush=True)
    x += step
