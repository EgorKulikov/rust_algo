use crate::string::slicelike::Slicelike;

pub trait Palindromes {
    fn odd_palindromes(&self) -> Vec<usize>;
    fn even_palindromes(&self) -> Vec<usize>;
}

impl<S: Slicelike + ?Sized> Palindromes for S
where
    S::Output: PartialEq + Sized,
{
    fn odd_palindromes(&self) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::with_capacity(self.len());
        let mut l = 0;
        let mut r = 0;
        for i in 0..self.len() {
            let mut k = if i >= r {
                1
            } else {
                res[l + r - 1 - i].min(r - i)
            };
            while i + k < self.len() && i >= k && self[i + k] == self[i - k] {
                k += 1;
            }
            res.push(k);
            if i + k > r {
                l = i + 1 - k;
                r = i + k;
            }
        }
        res
    }

    fn even_palindromes(&self) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::with_capacity(self.len());
        let mut l = 0;
        let mut r = 0;
        for i in 0..self.len() {
            let mut k = if i >= r { 0 } else { res[l + r - i].min(r - i) };
            while i + k < self.len() && i > k && self[i + k] == self[i - k - 1] {
                k += 1;
            }
            res.push(k);
            if i + k > r {
                l = i - k;
                r = i + k;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Palindromes;

    #[test]
    fn odd_abacaba() {
        assert_eq!(b"abacaba".odd_palindromes(), vec![1, 2, 1, 4, 1, 2, 1]);
    }

    #[test]
    fn even_abba() {
        assert_eq!(b"abba".even_palindromes(), vec![0, 0, 2, 0]);
    }

    #[test]
    fn odd_single() {
        assert_eq!(b"z".odd_palindromes(), vec![1]);
    }
}
