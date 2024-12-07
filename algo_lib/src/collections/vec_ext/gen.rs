pub trait VecGen<T> {
    fn gen(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec<T>;
    fn gen_append(&mut self, n: usize, f: impl FnMut(usize, &Self) -> T);
}

impl<T> VecGen<T> for Vec<T> {
    fn gen(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec<T> {
        let mut vec = Vec::with_capacity(n);
        vec.gen_append(n, f);
        vec
    }

    fn gen_append(&mut self, n: usize, mut f: impl FnMut(usize, &Self) -> T) {
        self.reserve(n);
        for i in 0..n {
            self.push(f(i, self));
        }
    }
}
