use crate::string::slicelike::Slicelike;

pub trait ZAlgorithm {
    fn z_algorithm(&self) -> Vec<usize>;
}

impl<S: Slicelike + ?Sized> ZAlgorithm for S
where
    S::Output: PartialEq + Sized,
{
    fn z_algorithm(&self) -> Vec<usize> {
        let mut res = Vec::with_capacity(self.len());
        res.push(0);
        let mut l = 0;
        let mut r = 0;
        for i in 1..self.len() {
            if i <= r {
                let cur = (r - i + 1).min(res[i - l]);
                res.push(cur);
            } else {
                res.push(0);
            }
            while i + res[i] < self.len() && self[res[i]] == self[i + res[i]] {
                res[i] += 1;
            }
            if i + res[i] - 1 > r {
                l = i;
                r = i + res[i] - 1;
            }
        }
        res
    }
}
