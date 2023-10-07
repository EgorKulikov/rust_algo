use crate::collections::slice_ext::legacy_fill::LegacyFill;
use std::mem;

#[derive(Default)]
enum Opt<T> {
    #[default]
    None,
    Some(u32, T),
}

impl<T> Opt<T> {
    fn index(&self) -> usize {
        match self {
            Opt::None => panic!("unreachable"),
            Opt::Some(index, _) => *index as usize,
        }
    }

    fn val(&self) -> &T {
        match self {
            Opt::None => panic!("unreachable"),
            Opt::Some(_, val) => val,
        }
    }

    fn set_index(&mut self, index: usize) {
        match self {
            Opt::None => panic!("unreachable"),
            Opt::Some(ind, _) => {
                *ind = index as u32;
            }
        }
    }

    fn set_val(&mut self, t: T) {
        match self {
            Opt::None => panic!("unreachable"),
            Opt::Some(_, val) => {
                *val = t;
            }
        }
    }

    fn take(&mut self) -> (usize, T) {
        let val = mem::take(self);
        match val {
            Opt::None => panic!("unreachable"),
            Opt::Some(ind, val) => (ind as usize, val),
        }
    }
}

impl<T> Clone for Opt<T> {
    fn clone(&self) -> Self {
        match self {
            Opt::None => Opt::None,
            Opt::Some(..) => panic!("unreachable"),
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
            pos: vec![Opt::None; capacity],
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
        match self.pos[el] {
            Opt::None => {
                self.pos[el] = Opt::Some(self.heap.len() as u32, val);
                self.heap.push(el as u32);
            }
            Opt::Some(..) => {
                self.pos[el].set_val(val);
            }
        }
        self.sift_up(self.pos[el].index());
        self.sift_down(self.pos[el].index());
    }

    pub fn add_or_relax(&mut self, el: usize, val: T) {
        match &self.pos[el] {
            Opt::None => {
                self.add_or_adjust(el, val);
            }
            Opt::Some(_, value) => {
                if &val < value {
                    self.add_or_adjust(el, val);
                }
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
        self.pos.legacy_fill(Opt::None);
    }

    pub fn remove(&mut self, el: usize) -> Option<T> {
        match self.pos[el] {
            Opt::None => None,
            Opt::Some(pos, _) => {
                let last = self.heap.pop().unwrap();
                let val = self.pos[last as usize].take().1;
                if self.is_empty() {
                    Some(val)
                } else {
                    let top_val = self.pos[el].take().1;
                    self.pos[last as usize] = Opt::Some(pos, val);
                    self.heap[pos as usize] = last;
                    self.sift_down(pos as usize);
                    self.sift_up(pos as usize);
                    Some(top_val)
                }
            }
        }
    }

    pub fn value(&self, el: usize) -> Option<&T> {
        match &self.pos[el] {
            Opt::None => None,
            Opt::Some(_, val) => Some(val),
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
