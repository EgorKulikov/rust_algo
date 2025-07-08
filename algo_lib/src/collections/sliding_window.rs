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
