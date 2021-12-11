//{"name":"A: And","group":"Facebook Coding Competitions - Facebook Hacker Cup 2021 Final Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2021/final-round/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n1 1\n1011011 101101\n2 0\n1011011 0\n0 101101\n2 1\n1011001 0\n0 101101\n3 3\n1 111\n111 1\n1 111\n5 1\n1101 11011\n11101 11111\n11011 1111\n10111 1001\n1011 10101\n5 2\n1101 11011\n11101 11111\n11011 1111\n10111 1001\n1011 10101\n","output":"Case #1: 10001000\nCase #2: 0\nCase #3: 1001\nCase #4: 1000\nCase #5: 1010\nCase #6: 11010\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"and_.*input[.]txt"},"output":{"type":"file","fileName":"and_output.txt","pattern":null},"languages":{"java":{"taskClass":"AAnd"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::io::Write;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        k: usize,
        pairs: Vec<(Str<'static>, Str<'static>)>,
        ans: Vec<u8>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.k = input.read();
            self.pairs = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            self.pairs
                .sort_by(|(s1, t1), (s2, t2)| s2.len().max(t2.len()).cmp(&s1.len().max(t1.len())));
            let len = self
                .pairs
                .last()
                .unwrap()
                .0
                .len()
                .max(self.pairs.last().unwrap().1.len());
            let mut bits_at = Vec::new();
            let mut set = (0..self.n).collect_vec();
            let n = self.n;
            let mut side = BitSet::new(n);
            let mut size = vec![1usize; n];
            let mut cost = vec![0usize; n];
            let mut sets = Vec::new();
            for i in 0..self.n {
                sets.push(vec![i]);
            }
            for i in (1..=len).rev() {
                let mut any = true;
                let mut at_left = Vec::new();
                let mut at_right = Vec::new();
                for j in 0..self.n {
                    let left_set = i <= self.pairs[j].0.len()
                        && self.pairs[j].0[self.pairs[j].0.len() - i] == b'1';
                    let right_set = i <= self.pairs[j].1.len()
                        && self.pairs[j].1[self.pairs[j].1.len() - i] == b'1';
                    if !left_set {
                        at_right.push(j);
                    }
                    if !right_set {
                        at_left.push(j);
                    }
                    if !left_set && !right_set {
                        any = false;
                    }
                }
                if !any {
                    continue;
                }
                if at_left.is_empty() && at_right.is_empty() {
                    bits_at.push(i);
                    continue;
                }
                let to_left_until = at_left.len();
                at_left.extend_from_slice(at_right.as_slice());
                let mut nset = set.clone();
                let mut nside = side.clone();
                let mut nsize = size.clone();
                let mut ncost = cost.clone();
                let mut nsets = sets.clone();
                let mut can = true;
                let aside = to_left_until == 0;
                let first = at_left[0];
                for (i, j) in at_left.into_iter().enumerate() {
                    let other_side = (i >= to_left_until) ^ aside;
                    if nset[first] == nset[j] {
                        if other_side != (nside[first] != nside[j]) {
                            can = false;
                            break;
                        }
                    } else {
                        let change_side = other_side != (nside[first] != nside[j]);
                        let (a, b) = if nsize[nset[first]] > nsize[nset[j]] {
                            (nset[first], nset[j])
                        } else {
                            (nset[j], nset[first])
                        };
                        let mut x = nsets[b].drain(..).collect_vec();
                        for k in x.drain(..) {
                            nset[k] = a;
                            nside.set(k, nside[k] ^ change_side);
                            nsets[a].push(k);
                            nsize[a] += 1;
                        }
                        ncost[a] += if change_side {
                            nsize[b] - ncost[b]
                        } else {
                            ncost[b]
                        };
                    }
                }
                if !can {
                    continue;
                }
                let mut total = 0usize;
                for i in 0..n {
                    if nset[i] == i {
                        total += ncost[i].min(nsize[i] - ncost[i]);
                    }
                }
                if total <= self.k {
                    bits_at.push(i - 1);
                    set = nset;
                    side = nside;
                    size = nsize;
                    cost = ncost;
                    sets = nsets;
                }
            }
            bits_at.sort();
            let mut ans = Vec::new();
            let mut cur = None;
            for i in bits_at {
                if cur.is_some() {
                    let c = cur.unwrap();
                    if c == i {
                        cur = Some(c + 1);
                    } else if i > c {
                        ans.push(c);
                        cur = Some(i);
                    } else {
                        ans.push(i);
                    }
                } else {
                    cur = Some(i);
                }
            }
            if let Some(c) = cur {
                ans.push(c);
            }
            if ans.is_empty() {
                self.ans = vec![b'0'];
            } else {
                self.ans = vec![b'0'; ans.last().unwrap() + 1];
                for i in ans {
                    self.ans[i] = b'1';
                }
                self.ans.reverse();
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out!(format!("Case #{}: ", test_case));
            output().write(self.ans.as_slice());
            out_line!();
        }
    }

    run_parallel::<Job>(input);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
