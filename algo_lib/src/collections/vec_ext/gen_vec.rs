use crate::collections::vec_ext::default::default_vec;

pub trait VecGen<T> {
    fn with_gen(n: usize, f: impl FnMut(usize) -> T) -> Vec<T>;
    fn with_gen_prefix(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec<T>;
    fn gen_append(&mut self, n: usize, f: impl FnMut(usize, &Self) -> T);

    fn with_gen_suffix(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec<T>
    where
        T: Default;
}

impl<T> VecGen<T> for Vec<T> {
    fn with_gen(n: usize, mut f: impl FnMut(usize) -> T) -> Vec<T> {
        Self::with_gen_prefix(n, |i, _| f(i))
    }

    fn with_gen_prefix(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec<T> {
        let mut vec = Vec::with_capacity(n);
        vec.gen_append(n, f);
        vec
    }

    fn gen_append(&mut self, n: usize, mut f: impl FnMut(usize, &Self) -> T) {
        self.reserve(n);
        let len = self.len();
        for i in 0..n {
            self.push(f(len + i, self));
        }
    }

    fn with_gen_suffix(n: usize, mut f: impl FnMut(usize, &Self) -> T) -> Vec<T>
    where
        T: Default,
    {
        let mut vec = default_vec(n);
        for i in (0..n).rev() {
            vec[i] = f(i, &vec);
        }
        vec
    }
}
