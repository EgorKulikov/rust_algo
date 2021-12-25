#![feature(map_first_last)]

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::{Input, Readable};
use std::collections::BTreeMap;

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

    let top = inp.read_table::<char>(3, 13);
    let bottom = inp.read_table::<char>(4, 9);

    let tp = |j| {
        if j < 4 {
            (top[(2, 3 + 2 * j)] as usize) - ('A' as usize)
        } else {
            (bottom[((j - 4) / 4, 1 + 2 * (j % 4))] as usize) - ('A' as usize)
        }
    };

    let mut res = BTreeMap::new();
    res.insert(0, 0);
    let mut pos = Vec::with_capacity(16);
    let mut edges = Vec::new();
    let mut ans = 0;
    while let Some((i, val)) = res.pop_first() {
        ans = val;
        pos.clear();
        edges.clear();
        let mut j = i;
        for _ in 0..16 {
            pos.push(j % 9);
            j /= 9;
        }
        for j in 0usize..16 {
            let t = tp(j);
            if pos[j] == 8 {
                continue;
            }
            let mut k = j;
            let mut bad = false;
            while k >= 4 {
                k -= 4;
                if pos[k] == 0 {
                    bad = true;
                    break;
                }
            }
            if bad {
                continue;
            }
            if pos[j] == 0 {
                let after = 2 + j % 4;
                for k in 0usize..7 {
                    let (left, right) = if k < after {
                        (k, after)
                    } else {
                        (after, k + 1)
                    };
                    let mut found = false;
                    for l in pos.iter().cloned() {
                        if l >= left + 1 && l < right + 1 {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        let mut steps = 2 + j / 4;
                        for l in left..(right - 1) {
                            if l == 0 || l == 5 {
                                steps += 1;
                            } else {
                                steps += 2;
                            }
                        }
                        edges.push((
                            i + (k + 1) * 9usize.pow(j as u32),
                            steps * 10usize.pow(t as u32),
                        ));
                    }
                }
                if j % 4 == t {
                    let mut bad = false;
                    let mut k = j + 4;
                    while k < 16 {
                        if pos[k] != 8 {
                            bad = true;
                            break;
                        }
                        k += 4;
                    }
                    if !bad {
                        edges.push((i + 8 * 9usize.pow(j as u32), 0));
                    }
                }
            } else if pos[j] < 8 {
                if pos[t] == 0 || pos[t + 4] == 0 || pos[t + 8] == 0 || pos[t + 12] == 0 {
                    continue;
                }
                let after = 2 + t;
                let (left, right) = if pos[j] - 1 < after {
                    (pos[j] - 1, after)
                } else {
                    (after, pos[j])
                };
                let mut found = false;
                for l in pos.iter().cloned() {
                    if l >= left + 1 && l < right + 1 && l != pos[j] {
                        found = true;
                        break;
                    }
                }
                if !found {
                    let mut steps = 5;
                    for l in left..(right - 1) {
                        if l == 0 || l == 5 {
                            steps += 1;
                        } else {
                            steps += 2;
                        }
                    }
                    for (k, l) in pos.iter().cloned().enumerate() {
                        if tp(k) == t && l == 8 {
                            steps -= 1;
                        }
                    }
                    edges.push((
                        i + (8 - pos[j]) * 9usize.pow(j as u32),
                        steps * 10usize.pow(t as u32),
                    ));
                }
            }
        }
        for (to, weight) in edges.iter().cloned() {
            if !res.contains_key(&to) {
                res.insert(to, val + weight);
            } else {
                res.get_mut(&to).unwrap().minim(val + weight);
            }
        }
    }
    println!("{}", ans);
}
