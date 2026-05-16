//{"name":"Win As Second","group":"Google Coding Competitions - Round 3 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008779b4/0000000000b4518a","interactive":true,"timeLimit":60000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WinAsSecond"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::collections::HashSet;

use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut nim = vec![0; 80];
    for i in 1..80 {
        let mut can = HashSet::new();
        for j in 0..i {
            for k in 1..=(i - j).min(3) {
                can.insert(nim[j] ^ nim[i - (j + k)]);
            }
        }
        for j in 0.. {
            if !can.contains(&j) {
                nim[i] = j;
                break;
            }
        }
    }
    let mut ans = vec![(0, 0, 0, 0); 11];
    let mut nim3 = Arr3d::new(40, 40, 40, 0);
    for i in 0..=27 {
        for j in 0..=1 {
            for k in 0..=1 {
                if i + j + k >= 40 {
                    break;
                }
                if i == 0 || j == 0 || k == 0 {
                    nim3[(i, j, k)] = nim[i + j + k + 1];
                    continue;
                }
                let mut can = HashSet::new();
                for l in 0..i {
                    for m in 1..=(i - l).min(3) {
                        can.insert(nim3[(l, j, k)] ^ nim[i - (l + m)]);
                    }
                }
                for l in 0..j {
                    for m in 1..=(j - l).min(3) {
                        can.insert(nim3[(i, l, k)] ^ nim[j - (l + m)]);
                    }
                }
                for l in 0..k {
                    for m in 1..=(k - l).min(3) {
                        can.insert(nim3[(i, j, l)] ^ nim[k - (l + m)]);
                    }
                }
                if i >= 2 {
                    can.insert(nim[i - 2] ^ nim[j] ^ nim[k]);
                }
                if j >= 2 {
                    can.insert(nim[i] ^ nim[j - 2] ^ nim[k]);
                }
                if k >= 2 {
                    can.insert(nim[i] ^ nim[j] ^ nim[k - 2]);
                }
                can.insert(nim[i] ^ nim[j] ^ nim[k]);
                can.insert(nim[i - 1] ^ nim[j] ^ nim[k]);
                can.insert(nim[i - 1] ^ nim[j - 1] ^ nim[k]);
                can.insert(nim[i] ^ nim[j - 1] ^ nim[k]);
                can.insert(nim[i] ^ nim[j] ^ nim[k - 1]);
                can.insert(nim[i - 1] ^ nim[j] ^ nim[k - 1]);
                can.insert(nim[i - 1] ^ nim[j - 1] ^ nim[k - 1]);
                can.insert(nim[i] ^ nim[j - 1] ^ nim[k - 1]);
                for l in 0.. {
                    if !can.contains(&l) {
                        nim3[(i, j, k)] = l;
                        if l == 0 {
                            let v = i + j + k + 1;
                            if v >= 30 && v <= 40 {
                                ans[v - 30] = (i, j, k, 0);
                            }
                            // println!("{i} {j} {k}");
                        }
                        break;
                    }
                }
            }
        }
    }
    let get = |mask: i32| -> i32 {
        let mask = i32::all_bits(30) - mask;
        let mut res = 0;
        let mut cur = 1;
        if mask.is_set(0) {
            let j = if mask.is_set(28) { 1 } else { 0 };
            let k = if mask.is_set(29) { 1 } else { 0 };
            let mut i = 0;
            while cur < 28 && mask.is_set(cur) {
                i += 1;
                cur += 1;
            }
            // eprintln!("nim3 {i} {j} {k}");
            res ^= nim3[(i, j, k)];
        } else {
            if mask.is_set(28) {
                res ^= nim[1];
            }
            if mask.is_set(29) {
                res ^= nim[1];
            }
        }
        while cur < 28 {
            let mut len = 0;
            while cur < 28 && mask.is_set(cur) {
                len += 1;
                cur += 1;
            }
            cur += 1;
            res ^= nim[len];
            // if len != 0 {
            //     eprintln!("nim {len}");
            // }
        }
        res
    };
    let _n = input.read_usize();
    let mut graph = Graph::new(30);
    for i in 1..=27 {
        out_line!(i, i + 1);
        graph.add_edge(i - 1, BiEdge::new(i));
    }
    out_line!(1, 29);
    graph.add_edge(0, BiEdge::new(28));
    out_line!(1, 30);
    graph.add_edge(0, BiEdge::new(29));
    let m = input.read_usize();
    for _ in 0..m {
        // eprintln!("Game");
        let mut red = 0;
        loop {
            let k = input.read_usize();
            // eprintln!("k = {k}");
            for _ in 0..k {
                let v = input.read_usize() - 1;
                red.set_bit(v);
                // eprintln!("v = {v}");
            }
            let mut found = false;
            // eprintln!("Move {}", get(red));
            for i in 0..30 {
                if red.is_set(i) {
                    continue;
                }
                for j in 0i32..(1 << graph[i].len()) {
                    let mut cur = red.with_bit(i);
                    let mut good = true;
                    for k in 0..graph[i].len() {
                        if j.is_set(k) && red.is_set(graph[i][k].to()) {
                            good = false;
                            break;
                        }
                        if j.is_set(k) {
                            cur.set_bit(graph[i][k].to());
                        }
                    }
                    if good && get(cur) == 0 {
                        out_line!(1 + j.count_ones());
                        out!(i + 1);
                        for k in 0..graph[i].len() {
                            if j.is_set(k) {
                                out!(format!(" {}", graph[i][k].to() + 1));
                            }
                        }
                        out_line!();
                        // eprintln!("Found");
                        found = true;
                        red = cur;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            assert!(found);
            if red == i32::all_bits(30) {
                break;
            }
        }
    }
}

#[test]
fn test() {
    let mut nim = vec![0; 80];
    for i in 1..80 {
        let mut can = HashSet::new();
        for j in 0..i {
            for k in 1..=(i - j).min(3) {
                can.insert(nim[j] ^ nim[i - (j + k)]);
            }
        }
        for j in 0.. {
            if !can.contains(&j) {
                nim[i] = j;
                break;
            }
        }
    }
    let mut ans = vec![(0, 0, 0, 0); 11];
    let mut nim3 = Arr3d::new(40, 40, 40, 0);
    for i in 0..40 {
        for j in 0..40 {
            for k in 0..40 {
                if i + j + k >= 40 {
                    break;
                }
                if i == 0 || j == 0 || k == 0 {
                    nim3[(i, j, k)] = nim[i + j + k + 1];
                    continue;
                }
                let mut can = HashSet::new();
                for l in 0..i {
                    for m in 1..=(i - l).min(3) {
                        can.insert(nim3[(l, j, k)] ^ nim[i - (l + m)]);
                    }
                }
                for l in 0..j {
                    for m in 1..=(j - l).min(3) {
                        can.insert(nim3[(i, l, k)] ^ nim[j - (l + m)]);
                    }
                }
                for l in 0..k {
                    for m in 1..=(k - l).min(3) {
                        can.insert(nim3[(i, j, l)] ^ nim[k - (l + m)]);
                    }
                }
                if i >= 2 {
                    can.insert(nim[i - 2] ^ nim[j] ^ nim[k]);
                }
                if j >= 2 {
                    can.insert(nim[i] ^ nim[j - 2] ^ nim[k]);
                }
                if k >= 2 {
                    can.insert(nim[i] ^ nim[j] ^ nim[k - 2]);
                }
                can.insert(nim[i] ^ nim[j] ^ nim[k]);
                can.insert(nim[i - 1] ^ nim[j] ^ nim[k]);
                can.insert(nim[i - 1] ^ nim[j - 1] ^ nim[k]);
                can.insert(nim[i] ^ nim[j - 1] ^ nim[k]);
                can.insert(nim[i] ^ nim[j] ^ nim[k - 1]);
                can.insert(nim[i - 1] ^ nim[j] ^ nim[k - 1]);
                can.insert(nim[i - 1] ^ nim[j - 1] ^ nim[k - 1]);
                can.insert(nim[i] ^ nim[j - 1] ^ nim[k - 1]);
                for l in 0.. {
                    if !can.contains(&l) {
                        nim3[(i, j, k)] = l;
                        if i == 16 && j == 13 && k == 3 {
                            println!("{l}");
                        }
                        if l == 0 {
                            let v = i + j + k + 1;
                            if v >= 30 && v <= 40 {
                                ans[v - 30] = (i, j, k, 0);
                            }
                            // println!("{i} {j} {k}");
                        }
                        break;
                    }
                }
            }
        }
    }
    /*    let mut nim4 = Arr4d::new(40, 40, 40, 40, 0);
    for i in 0..40 {
        for j in 0..40 {
            for k in 0..40 {
                for n in 0..40 {
                    if i + j + k + n >= 40 {
                        break;
                    }
                    if i == 0 {
                        nim4[(i, j, k, n)] = nim3[(j, k, n)];
                        continue;
                    }
                    if j == 0 {
                        nim4[(i, j, k, n)] = nim3[(i, k, n)];
                        continue;
                    }
                    if k == 0 {
                        nim4[(i, j, k, n)] = nim3[(i, j, n)];
                        continue;
                    }
                    if n == 0 {
                        nim4[(i, j, k, n)] = nim3[(i, j, k)];
                        continue;
                    }
                    let mut can = HashSet::new();
                    for l in 0..i {
                        for m in 1..=(i - l).min(3) {
                            can.insert(nim4[(l, j, k, n)] ^ nim[i - (l + m)]);
                        }
                    }
                    for l in 0..j {
                        for m in 1..=(j - l).min(3) {
                            can.insert(nim4[(i, l, k, n)] ^ nim[j - (l + m)]);
                        }
                    }
                    for l in 0..k {
                        for m in 1..=(k - l).min(3) {
                            can.insert(nim4[(i, j, l, n)] ^ nim[k - (l + m)]);
                        }
                    }
                    for l in 0..n {
                        for m in 1..=(n - l).min(3) {
                            can.insert(nim4[(i, j, k, l)] ^ nim[n - (l + m)]);
                        }
                    }
                    if i >= 2 {
                        can.insert(nim[i - 2] ^ nim[j] ^ nim[k] ^ nim[n]);
                    }
                    if j >= 2 {
                        can.insert(nim[i] ^ nim[j - 2] ^ nim[k] ^ nim[n]);
                    }
                    if k >= 2 {
                        can.insert(nim[i] ^ nim[j] ^ nim[k - 2] ^ nim[n]);
                    }
                    if n >= 2 {
                        can.insert(nim[i] ^ nim[j] ^ nim[k] ^ nim[n - 2]);
                    }
                    can.insert(nim[i] ^ nim[j] ^ nim[k] ^ nim[n]);
                    can.insert(nim[i - 1] ^ nim[j] ^ nim[k] ^ nim[n]);
                    can.insert(nim[i - 1] ^ nim[j - 1] ^ nim[k] ^ nim[n]);
                    can.insert(nim[i] ^ nim[j - 1] ^ nim[k] ^ nim[n]);
                    can.insert(nim[i] ^ nim[j] ^ nim[k - 1] ^ nim[n]);
                    can.insert(nim[i - 1] ^ nim[j] ^ nim[k - 1] ^ nim[n]);
                    can.insert(nim[i - 1] ^ nim[j - 1] ^ nim[k - 1] ^ nim[n]);
                    can.insert(nim[i] ^ nim[j - 1] ^ nim[k - 1] ^ nim[n]);
                    can.insert(nim[i] ^ nim[j] ^ nim[k] ^ nim[n - 1]);
                    can.insert(nim[i - 1] ^ nim[j] ^ nim[k] ^ nim[n - 1]);
                    can.insert(nim[i - 1] ^ nim[j - 1] ^ nim[k] ^ nim[n - 1]);
                    can.insert(nim[i] ^ nim[j - 1] ^ nim[k] ^ nim[n - 1]);
                    can.insert(nim[i] ^ nim[j] ^ nim[k - 1] ^ nim[n - 1]);
                    can.insert(nim[i - 1] ^ nim[j] ^ nim[k - 1] ^ nim[n - 1]);
                    can.insert(nim[i - 1] ^ nim[j - 1] ^ nim[k - 1] ^ nim[n - 1]);
                    can.insert(nim[i] ^ nim[j - 1] ^ nim[k - 1] ^ nim[n - 1]);
                    for l in 0.. {
                        if !can.contains(&l) {
                            nim4[(i, j, k, n)] = l;
                            if l == 0 {
                                let v = i + j + k + n + 1;
                                if v >= 30 && v <= 40 {
                                    ans[v - 30] = (i, j, k, n);
                                }
                                // println!("{i} {j} {k}");
                            }
                            break;
                        }
                    }
                }
            }
        }
    }*/
    println!("{:?}", ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
