use crate::collections::iter_ext::IterExt;

pub fn string_to_card(s: &str) -> (usize, usize) {
    let mut chars = s.chars();
    let rank = "23456789TJQKA".find(chars.next().unwrap()).unwrap();
    let suit = "CDHS".find(chars.next().unwrap()).unwrap();
    (rank, suit)
}

pub fn card_to_string(rank: usize, suit: usize) -> String {
    format!(
        "{}{}",
        "23456789TJQKA".chars().collect_vec()[rank],
        "CDHS".chars().collect_vec()[suit]
    )
}
