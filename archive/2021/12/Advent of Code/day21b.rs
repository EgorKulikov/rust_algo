use algo_lib::collections::arr4d::Arr4d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};

pub trait CommaList {
    fn read_list<T: Readable>(&mut self) -> Vec<T>;
}

impl CommaList for Input<'_> {
    fn read_list<T: Readable>(&mut self) -> Vec<T> {
        let mut s: String = self.read();
        s = s.replace(",", " ");
        let mut b = s.as_bytes();
        let input = Input::new(&mut b);
        input.into_iter().collect_vec()
    }
}

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    for _ in 0..4 {
        inp.next_token();
    }
    let first: usize = inp.read();
    for _ in 0..4 {
        inp.next_token();
    }
    let second: usize = inp.read();

    let mut qty = Arr4d::new(21, 21, 11, 11, 0u128);
    qty[(0, 0, first, second)] = 1;
    let mut next = Arr4d::new(21, 21, 11, 11, 0u128);
    let mut first_win = 0;
    let mut second_win = 0;
    while qty.iter().sum::<u128>() != 0 {
        next.fill(0);
        for i in 0..=20 {
            for j in 0..=20 {
                for k in 1..=10 {
                    for l in 1..=10 {
                        for a in 1..=3 {
                            for b in 1..=3 {
                                for c in 1..=3 {
                                    let mut nk = k + a + b + c;
                                    if nk > 10 {
                                        nk -= 10;
                                    }
                                    if i + nk >= 21 {
                                        first_win += qty[(i, j, k, l)];
                                    } else {
                                        next[(i + nk, j, nk, l)] += qty[(i, j, k, l)];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        qty.fill(0);
        for i in 0..=20 {
            for j in 0..=20 {
                for k in 1..=10 {
                    for l in 1..=10 {
                        for a in 1..=3 {
                            for b in 1..=3 {
                                for c in 1..=3 {
                                    let mut nl = l + a + b + c;
                                    if nl > 10 {
                                        nl -= 10;
                                    }
                                    if j + nl >= 21 {
                                        second_win += next[(i, j, k, l)];
                                    } else {
                                        qty[(i, j + nl, k, nl)] += next[(i, j, k, l)];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", first_win.max(second_win));
}
