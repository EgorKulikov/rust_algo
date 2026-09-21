pub trait StrSplit {
    fn str_split(&self, pattern: &[u8]) -> Vec<&[u8]>;
}

impl StrSplit for [u8] {
    fn str_split(&self, pattern: &[u8]) -> Vec<&[u8]> {
        let mut res = Vec::new();
        let mut start = 0;
        for i in 0..self.len() {
            // `i < start` is inside the previous occurrence.
            if i >= start && self[i..].starts_with(pattern) {
                res.push(&self[start..i]);
                start = i + pattern.len();
            }
        }
        res.push(&self[start..]);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::StrSplit;

    fn reference(s: &str, pattern: &str) -> Vec<Vec<u8>> {
        s.split(pattern).map(|x| x.as_bytes().to_vec()).collect()
    }

    #[test]
    fn overlapping_occurrences() {
        let s: &[u8] = b"aaa";
        assert_eq!(s.str_split(b"aa"), vec![&b""[..], &b"a"[..]]);
    }

    #[test]
    fn matches_std_split() {
        for s in ["", "a", "abab", "aaaa", "xaaxaaax", "abcabca", "aabaabaa"] {
            for pattern in ["a", "aa", "ab", "aba", "abc", "x"] {
                let got: Vec<Vec<u8>> = s
                    .as_bytes()
                    .str_split(pattern.as_bytes())
                    .into_iter()
                    .map(|x| x.to_vec())
                    .collect();
                assert_eq!(got, reference(s, pattern), "{:?} by {:?}", s, pattern);
            }
        }
    }
}
