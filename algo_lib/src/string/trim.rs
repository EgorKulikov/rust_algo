// 1.80
pub trait StrTrim {
    fn trim(&self) -> &[u8];
}

impl StrTrim for [u8] {
    fn trim(&self) -> &[u8] {
        let mut start = 0;
        while start < self.len() && self[start].is_ascii_whitespace() {
            start += 1;
        }
        let mut end = self.len();
        while end > start && self[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        &self[start..end]
    }
}
