use crate::collections::slice_ext::bounds::Bounds;
use std::mem::MaybeUninit;

pub fn compress<T: Ord + Clone, const N: usize>(vs: [&[T]; N]) -> (Vec<T>, [Vec<usize>; N]) {
    let mut size = 0;
    for v in &vs {
        size += v.len();
    }
    let mut all = Vec::with_capacity(size);
    for v in &vs {
        all.extend_from_slice(v);
    }
    all.sort();
    all.dedup();
    let arrs = unsafe {
        let mut arr: MaybeUninit<[Vec<usize>; N]> = MaybeUninit::uninit();
        for (i, v) in vs.iter().enumerate() {
            let mut cur = Vec::with_capacity(vs[i].len());
            for vv in *v {
                cur.push(all.bin_search(vv).unwrap());
            }
            arr.as_mut_ptr().cast::<Vec<usize>>().add(i).write(cur);
        }
        arr.assume_init()
    };
    (all, arrs)
}
