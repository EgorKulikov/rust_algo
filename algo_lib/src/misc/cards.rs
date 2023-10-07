use crate::collections::iter_ext::find_count::IterFindCount;
use crate::string::str::Str;

pub fn string_to_card(s: &Str) -> (usize, usize) {
    assert!(s.len() == 2);
    let rank = b"23456789TJQKA".iter().find_eq(&s[0]).unwrap();
    let suit = b"CDHS".iter().find_eq(&s[1]).unwrap();
    (rank, suit)
}

pub fn card_to_string(rank: usize, suit: usize) -> Str<'static> {
    let mut res = Str::with_capacity(2);
    res.push(b"23456789TJQKA"[rank]);
    res.push(b"CDHS"[suit]);
    res
}
