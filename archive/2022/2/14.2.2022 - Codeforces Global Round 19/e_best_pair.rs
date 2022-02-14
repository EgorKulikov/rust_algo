//{"name":"E. Best Pair","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6 1\n6 3 6 7 3 3\n3 6\n2 0\n3 4\n7 4\n1 2 2 3 1 5 1\n1 5\n3 5\n1 3\n2 5\n","output":"40\n14\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBestPair"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::{BinaryHeap, HashSet};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_usize_vec(n);
    let bad = input
        .read_usize_pair_vec(m)
        .into_iter()
        .collect::<HashSet<_>>();

    let mut qty: DefaultMap<_, usize> = DefaultMap::new();
    for i in a {
        qty[i] += 1;
    }
    let mut by_qty: DefaultMap<_, Vec<_>> = DefaultMap::new();
    for (v, q) in qty {
        by_qty[q].push(v);
    }
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
    struct Candidate {
        value: usize,
        left_qty: usize,
        left_pos: usize,
        right_qty: usize,
        right_pos: usize,
        left_val: usize,
        right_val: usize,
    }

    impl Candidate {
        pub fn new(
            left_qty: usize,
            left_val: usize,
            left_pos: usize,
            right_qty: usize,
            right_val: usize,
            right_pos: usize,
        ) -> Self {
            Self {
                value: (left_qty + right_qty) * (left_val + right_val),
                left_qty,
                left_pos,
                right_qty,
                right_pos,
                left_val,
                right_val,
            }
        }

        fn pair(&self) -> (usize, usize) {
            (
                self.left_val.min(self.right_val),
                self.left_val.max(self.right_val),
            )
        }
    }

    let mut heap = BinaryHeap::new();
    let mut set = HashSet::new();
    for v in by_qty.values_mut() {
        v.sort_unstable();
        v.reverse();
    }
    fn add(heap: &mut BinaryHeap<Candidate>, set: &mut HashSet<Candidate>, cand: Candidate) {
        if !set.contains(&cand) {
            set.insert(cand);
            heap.push(cand);
        }
    }
    for (&q, v) in by_qty.iter() {
        if v.len() > 1 {
            add(&mut heap, &mut set, Candidate::new(q, v[0], 0, q, v[1], 1));
        }
        for (&q2, v2) in by_qty.iter() {
            if q2 == q {
                break;
            }
            add(
                &mut heap,
                &mut set,
                Candidate::new(q, v[0], 0, q2, v2[0], 0),
            );
        }
    }
    while let Some(cand) = heap.pop() {
        if !bad.contains(&cand.pair()) {
            out_line!(cand.value);
            return;
        }
        if cand.left_qty == cand.right_qty {
            if cand.left_pos + 1 == cand.right_pos {
                if cand.right_pos + 1 < by_qty[cand.left_qty].len() {
                    add(
                        &mut heap,
                        &mut set,
                        Candidate::new(
                            cand.left_qty,
                            cand.right_val,
                            cand.right_pos,
                            cand.left_qty,
                            by_qty[cand.left_qty][cand.right_pos + 1],
                            cand.right_pos + 1,
                        ),
                    );
                }
            } else {
                add(
                    &mut heap,
                    &mut set,
                    Candidate::new(
                        cand.left_qty,
                        by_qty[cand.left_qty][cand.left_pos + 1],
                        cand.left_pos + 1,
                        cand.left_qty,
                        cand.right_val,
                        cand.right_pos,
                    ),
                );
            }
            if cand.right_pos + 1 < by_qty[cand.left_qty].len() {
                add(
                    &mut heap,
                    &mut set,
                    Candidate::new(
                        cand.left_qty,
                        cand.left_val,
                        cand.left_pos,
                        cand.left_qty,
                        by_qty[cand.left_qty][cand.right_pos + 1],
                        cand.right_pos + 1,
                    ),
                );
            }
        } else {
            if cand.left_pos + 1 < by_qty[cand.left_qty].len() {
                add(
                    &mut heap,
                    &mut set,
                    Candidate::new(
                        cand.left_qty,
                        by_qty[cand.left_qty][cand.left_pos + 1],
                        cand.left_pos + 1,
                        cand.right_qty,
                        cand.right_val,
                        cand.right_pos,
                    ),
                );
            }
            if cand.right_pos + 1 < by_qty[cand.right_qty].len() {
                add(
                    &mut heap,
                    &mut set,
                    Candidate::new(
                        cand.left_qty,
                        cand.left_val,
                        cand.left_pos,
                        cand.right_qty,
                        by_qty[cand.right_qty][cand.right_pos + 1],
                        cand.right_pos + 1,
                    ),
                );
            }
        }
    }
    unreachable!();
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
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
