use crate::collections::vec_ext::default::default_vec;
use crate::misc::maybe::Maybe;

struct Opt<T> {
    index: u32,
    value: Maybe<T>,
}

#[allow(clippy::derivable_impls)]
impl<T> Default for Opt<T> {
    fn default() -> Self {
        Self {
            index: u32::MAX,
            value: Maybe::none(),
        }
    }
}

impl<T> Opt<T> {
    fn index(&self) -> usize {
        assert!(self.index != u32::MAX);
        self.index as usize
    }

    fn val(&self) -> &T {
        assert!(self.index != u32::MAX);
        unsafe { self.value.as_ref() }
    }

    fn set_index(&mut self, index: usize) {
        assert!(self.index != u32::MAX);
        self.index = index as u32;
    }

    fn set_val(&mut self, t: T) {
        assert!(self.index != u32::MAX);
        unsafe { *self.value.as_mut() = t };
    }

    fn take(&mut self) -> (usize, T) {
        assert!(self.index != u32::MAX);
        let value = unsafe { self.value.take() };
        let index = self.index as usize;
        self.index = u32::MAX;
        (index, value)
    }

    fn is_none(&self) -> bool {
        self.index == u32::MAX
    }
}

impl<T> Drop for Opt<T> {
    fn drop(&mut self) {
        if self.index != u32::MAX {
            unsafe { self.value.drop() }
        }
    }
}

pub struct IndexedHeap<T> {
    heap: Vec<u32>,
    pos: Vec<Opt<T>>,
}

impl<T: PartialOrd> IndexedHeap<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            heap: Vec::with_capacity(capacity),
            pos: default_vec(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.heap.iter().map(|x| *x as usize)
    }

    pub fn add_or_adjust(&mut self, el: usize, val: T) {
        if self.pos[el].is_none() {
            self.pos[el].index = self.heap.len() as u32;
            self.pos[el].value = Maybe::new(val);
            self.heap.push(el as u32);
            self.sift_up(self.pos[el].index());
        } else {
            let v = self.pos[el].val();
            let less = *v < val;
            self.pos[el].set_val(val);
            if less {
                self.sift_down(self.pos[el].index());
            } else {
                self.sift_up(self.pos[el].index());
            }
        }
    }

    pub fn add_or_relax(&mut self, el: usize, val: T) {
        if self.pos[el].is_none() {
            self.add_or_adjust(el, val);
        } else {
            let value = self.pos[el].val();
            if &val < value {
                self.add_or_adjust(el, val);
            }
        }
    }

    pub fn peek(&self) -> Option<(usize, &T)> {
        if self.is_empty() {
            None
        } else {
            let at = self.heap[0] as usize;
            Some((self.pos[at].index(), self.pos[at].val()))
        }
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        if self.is_empty() {
            None
        } else {
            let el = self.heap[0] as usize;
            Some((el, self.remove(el).unwrap()))
        }
    }

    pub fn clear(&mut self) {
        self.heap.clear();
        for el in &mut self.pos {
            *el = Opt::default();
        }
    }

    pub fn remove(&mut self, el: usize) -> Option<T> {
        if self.pos[el].is_none() {
            None
        } else {
            let pos = self.pos[el].index();
            let last = self.heap.pop().unwrap();
            let val = self.pos[last as usize].take().1;
            if self.is_empty() {
                Some(val)
            } else {
                let top_val = self.pos[el].take().1;
                self.pos[last as usize].index = pos as u32;
                self.pos[last as usize].value = Maybe::new(val);
                self.heap[pos] = last;
                self.sift_down(pos);
                self.sift_up(pos);
                Some(top_val)
            }
        }
    }

    pub fn value(&self, el: usize) -> Option<&T> {
        if self.pos[el].is_none() {
            None
        } else {
            Some(self.pos[el].val())
        }
    }

    fn sift_up(&mut self, mut index: usize) {
        let v = self.heap[index] as usize;
        while index > 0 {
            let parent = (index - 1) >> 1;
            let par_val = self.heap[parent] as usize;
            if self.pos[par_val].val() <= self.pos[v].val() {
                self.heap[index] = v as u32;
                self.pos[v].set_index(index);
                return;
            }
            self.heap[index] = par_val as u32;
            self.pos[par_val].set_index(index);
            index = parent;
        }
        self.heap[0] = v as u32;
        self.pos[v].set_index(0);
    }

    fn sift_down(&mut self, mut index: usize) {
        let v = self.heap[index] as usize;
        loop {
            let mut child = (index << 1) + 1;
            if child >= self.len() {
                break;
            }
            if child + 1 < self.len()
                && self.pos[self.heap[child] as usize].val()
                    > self.pos[self.heap[child + 1] as usize].val()
            {
                child += 1;
            }
            if self.pos[self.heap[child] as usize].val() >= self.pos[v].val() {
                break;
            }
            self.heap[index] = self.heap[child];
            self.pos[self.heap[index] as usize].set_index(index);
            index = child;
        }
        self.heap[index] = v as u32;
        self.pos[v].set_index(index);
    }
}
