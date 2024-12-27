use std::cell::Cell;
use std::marker::PhantomData;

pub struct ExpressionParser<T, Parse>
where
    Parse: Fn(&[u8]) -> T,
{
    max_priority: usize,
    binary_ops: Vec<(usize, u8, Box<dyn Fn(T, T) -> T>)>,
    unary_ops: Vec<(u8, Box<dyn Fn(T) -> T>)>,
    binary_id: Vec<Option<usize>>,
    unary_id: Vec<Option<usize>>,
    parse: Parse,
    pos: Cell<usize>,
    _marker: PhantomData<T>,
}

impl<T, Parse> ExpressionParser<T, Parse>
where
    Parse: Fn(&[u8]) -> T,
{
    pub fn new(parse: Parse) -> Self {
        Self {
            max_priority: 0,
            binary_ops: Vec::new(),
            unary_ops: Vec::new(),
            binary_id: vec![None; 256],
            unary_id: vec![None; 256],
            parse,
            pos: Cell::new(0),
            _marker: PhantomData,
        }
    }

    pub fn add_binary_op(&mut self, priority: usize, op: u8, f: impl Fn(T, T) -> T + 'static) {
        self.max_priority = self.max_priority.max(priority + 1);
        self.binary_id[op as usize] = Some(self.binary_ops.len());
        self.binary_ops.push((priority, op, Box::new(f)));
    }

    pub fn add_unary_op(&mut self, op: u8, f: impl Fn(T) -> T + 'static) {
        self.unary_id[op as usize] = Some(self.unary_ops.len());
        self.unary_ops.push((op, Box::new(f)));
    }

    pub fn parse(&self, s: &[u8]) -> T {
        self.pos.set(0);
        self.parse_expr(0, s)
    }

    fn parse_expr(&self, priority: usize, s: &[u8]) -> T {
        if priority == self.max_priority {
            self.parse_unary(s)
        } else {
            let mut res = self.parse_expr(priority + 1, s);
            loop {
                if self.pos.get() == s.len() {
                    return res;
                }
                let mut found = false;
                if let Some(id) = self.binary_id[s[self.pos.get()] as usize] {
                    if self.binary_ops[id].0 == priority {
                        self.pos.set(self.pos.get() + 1);
                        let other = self.parse_expr(priority + 1, s);
                        res = (self.binary_ops[id].2)(res, other);
                        found = true;
                    }
                }
                if !found {
                    break;
                }
            }
            res
        }
    }

    fn parse_unary(&self, s: &[u8]) -> T {
        while s[self.pos.get()] == b' ' {
            self.pos.set(self.pos.get() + 1);
        }
        if let Some(id) = self.unary_id[s[self.pos.get()] as usize] {
            self.pos.set(self.pos.get() + 1);
            let other = self.parse_unary(s);
            return (self.unary_ops[id].1)(other);
        }
        if s[self.pos.get()] == b'(' {
            self.pos.set(self.pos.get() + 1);
            let res = self.parse_expr(0, s);
            self.pos.set(self.pos.get() + 1);
            return res;
        }
        self.parse_atom(s)
    }

    fn parse_atom(&self, s: &[u8]) -> T {
        let start = self.pos.get();
        loop {
            if self.pos.get() == s.len()
                || s[self.pos.get()] == b')'
                || self.binary_id[s[self.pos.get()] as usize].is_some()
            {
                break;
            }
            self.pos.set(self.pos.get() + 1);
        }
        let s = &s[start..self.pos.get()];
        let s = s.trim_ascii();
        (self.parse)(s)
    }
}
