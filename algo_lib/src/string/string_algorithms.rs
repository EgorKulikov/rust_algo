pub trait StringAlgorithms {
    fn z_algorithm(&self) -> Vec<usize>;
    fn prefix_function(&self) -> Vec<usize>;
    fn odd_palindromes(&self) -> Vec<usize>;
    fn even_palindromes(&self) -> Vec<usize>;
}

impl<T: PartialEq> StringAlgorithms for &[T] {
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
