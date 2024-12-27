use crate::string::str::Str;

pub trait StrConcat {
    fn str_concat(&self, other: &[u8]) -> Str;
}

impl StrConcat for [u8] {
    fn str_concat(&self, other: &[u8]) -> Str {
        let mut res = Vec::with_capacity(self.len() + other.len());
        res.extend_from_slice(self);
        res.extend_from_slice(other);
        Str::from(res)
    }
}
