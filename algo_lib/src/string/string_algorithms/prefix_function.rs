use crate::string::slicelike::Slicelike;

pub trait PrefixFunction {
    fn prefix_function(&self) -> Vec<usize>;
}

impl<S: Slicelike + ?Sized> PrefixFunction for S
    where
        S::Output: PartialEq + Sized,
{
    fn prefix_function(&self) -> Vec<usize> {
        let mut res = Vec::with_capacity(self.len());
        res.push(0);
        for i in 1..self.len() {
            let mut j = res[i - 1];
            while j > 0 && self[i] != self[j] {
                j = res[j - 1];
            }
            if self[i] == self[j] {
                j += 1;
            }
            res.push(j);
        }
        res
    }
}
