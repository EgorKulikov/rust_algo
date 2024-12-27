pub trait StrSplit {
    fn str_split(&self, pattern: &[u8]) -> Vec<&[u8]>;
}

impl StrSplit for [u8] {
    fn str_split(&self, pattern: &[u8]) -> Vec<&[u8]> {
        let mut res = Vec::new();
        let mut start = 0;
        for i in 0..self.len() {
            if self[i..].starts_with(pattern) {
                res.push(&self[start..i]);
                start = i + pattern.len();
            }
        }
        res.push(&self[start..]);
        res
    }
}
