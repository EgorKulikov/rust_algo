pub fn create_order(n: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        res.push(i);
    }
    res
}

pub trait MinimMaxim: PartialOrd + Sized {
    fn minim(&mut self, other: Self) -> bool {
        if other < *self {
            *self = other;
            true
        } else {
            false
        }
    }

    fn maxim(&mut self, other: Self) -> bool {
        if other > *self {
            *self = other;
            true
        } else {
            false
        }
    }
}

impl<T: PartialOrd + Sized> MinimMaxim for T {}
