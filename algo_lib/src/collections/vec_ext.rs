use std::mem::MaybeUninit;

pub trait Qty {
    fn qty_bound(&self, bound: usize) -> Vec<usize>;
    fn qty(&self) -> Vec<usize>;
}

impl Qty for &[usize] {
    fn qty_bound(&self, bound: usize) -> Vec<usize> {
        let mut res = vec![0; bound];
        for i in self.iter() {
            res[*i] += 1;
        }
        res
    }

    fn qty(&self) -> Vec<usize> {
        if self.is_empty() {
            Vec::new()
        } else {
            self.qty_bound(self.iter().max().unwrap() + 1)
        }
    }
}

pub trait Bounds<T: Ord> {
    fn lower_bound(&self, el: &T) -> usize;
    fn upper_bound(&self, el: &T) -> usize;
    fn bin_search(&self, el: &T) -> Option<usize>;
}

impl<T: Ord> Bounds<T> for &[T] {
    fn lower_bound(&self, el: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if &self[mid] < el {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn upper_bound(&self, el: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if &self[mid] <= el {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn bin_search(&self, el: &T) -> Option<usize> {
        let at = self.lower_bound(el);
        if at == self.len() || &self[at] != el {
            None
        } else {
            Some(at)
        }
    }
}

pub fn compress<T: Eq + Ord + Clone, const N: usize>(vs: [&[T]; N]) -> (Vec<T>, [Vec<usize>; N]) {
    let mut size = 0;
    for v in vs {
        size += v.len();
    }
    let mut all = Vec::with_capacity(size);
    for v in vs {
        for a in v {
            all.push(a.clone());
        }
    }
    all.sort();
    all.dedup();
    let mut res: MaybeUninit<[Vec<usize>; N]> = std::mem::MaybeUninit::uninit();
    let mut ptr = res.as_mut_ptr() as *mut Vec<usize>;
    for i in 0..N {
        let mut cur = Vec::with_capacity(vs[i].len());
        for j in 0..vs[i].len() {
            cur.push((&all[..]).bin_search(&vs[i][j]).unwrap());
        }
        unsafe {
            ptr.write(cur);
            ptr = ptr.add(1);
        }
    }
    (all, unsafe { res.assume_init() })
}
