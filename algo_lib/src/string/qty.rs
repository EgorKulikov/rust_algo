pub trait StrQty {
    fn qty(&self, from: u8, to: u8) -> Vec<usize>;
    fn qty_lower(&self) -> Vec<usize>;
}

impl StrQty for [u8] {
    fn qty(&self, from: u8, to: u8) -> Vec<usize> {
        let mut res = vec![0; (to - from + 1) as usize];
        for &c in self {
            res[(c - from) as usize] += 1;
        }
        res
    }

    fn qty_lower(&self) -> Vec<usize> {
        self.qty(b'a', b'z')
    }
}
