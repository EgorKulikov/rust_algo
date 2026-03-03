pub struct SlidingWindow<T, F> {
    comb: F,
    size: usize,
    window: Vec<T>,
    left_cum: Vec<T>,
    right_cum: Vec<T>,
}

impl<T: Copy, F: Fn(T, T) -> T> SlidingWindow<T, F> {
    pub fn new(size: usize, comb: F) -> Self {
        Self {
            comb,
            size,
            window: Vec::with_capacity(size),
            left_cum: Vec::with_capacity(size),
            right_cum: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, val: T) {
        self.window.push(val);
        self.right_cum.push(if self.right_cum.is_empty() {
            val
        } else {
            (self.comb)(*self.right_cum.last().unwrap(), val)
        });
        if self.window.len() == self.size {
            self.left_cum.clear();
            self.left_cum.push(self.window[self.size - 1]);
            for i in 1..self.size {
                self.left_cum.push((self.comb)(
                    self.window[self.size - 1 - i],
                    self.left_cum[i - 1],
                ));
            }
            self.window.clear();
            self.right_cum.clear();
        }
    }

    pub fn get(&self) -> T {
        if self.left_cum.is_empty() {
            *self.right_cum.last().unwrap()
        } else if self.right_cum.is_empty() {
            *self.left_cum.last().unwrap()
        } else {
            (self.comb)(
                self.left_cum[self.size - self.window.len() - 1],
                *self.right_cum.last().unwrap(),
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::SlidingWindow;

    #[test]
    fn sliding_sum() {
        let mut sw = SlidingWindow::new(3, |a: i64, b: i64| a + b);
        for v in 1..=3 {
            sw.push(v);
        }
        assert_eq!(sw.get(), 6); // 1+2+3
        sw.push(4);
        assert_eq!(sw.get(), 9); // 2+3+4
        sw.push(5);
        assert_eq!(sw.get(), 12); // 3+4+5
        sw.push(6);
        assert_eq!(sw.get(), 15); // 4+5+6
    }

    #[test]
    fn sliding_min() {
        let mut sw = SlidingWindow::new(3, |a: i64, b: i64| a.min(b));
        for &v in &[5i64, 3, 8] {
            sw.push(v);
        }
        assert_eq!(sw.get(), 3); // min(5,3,8)
        sw.push(1);
        assert_eq!(sw.get(), 1); // min(3,8,1)
        sw.push(7);
        assert_eq!(sw.get(), 1); // min(8,1,7)
        sw.push(2);
        assert_eq!(sw.get(), 1); // min(1,7,2)
    }
}
