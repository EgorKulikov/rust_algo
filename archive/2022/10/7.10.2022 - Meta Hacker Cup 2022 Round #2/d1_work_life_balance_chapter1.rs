//{"name":"D1: Work-Life Balance - Chapter 1","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/D1","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2 2\n1 2\n2 1 1\n1 3 1\n4 2\n1 1 3 3\n4 1 3\n1 3 2\n6 4\n1 2 3 3 3 3\n6 2 3\n4 1 2\n3 1 2\n1 3 5\n","output":"Case #1: -1\nCase #2: 1\nCase #3: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"worklife_balance_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"worklife_balance_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"D1WorkLifeBalanceChapter1"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        m: usize,
        a: Vec<usize>,
        q: Vec<(usize, usize, usize)>,
        ans: i64,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.m = input.read();
            self.a = input.read_vec(self.n);
            self.q = input.read_vec(self.m);
        }

        fn solve(&mut self) {
            let mut ones = FenwickTree::new(self.n);
            let mut twos = FenwickTree::new(self.n);
            let mut threes = FenwickTree::new(self.n);
            for (i, &c) in self.a.iter().enumerate() {
                if c == 1 {
                    ones.add(i, 1);
                } else if c == 2 {
                    twos.add(i, 1);
                } else {
                    threes.add(i, 1);
                }
            }
            for &(x, y, z) in &self.q {
                if self.a[x - 1] == 1 {
                    ones.add(x - 1, -1);
                } else if self.a[x - 1] == 2 {
                    twos.add(x - 1, -1);
                } else {
                    threes.add(x - 1, -1);
                }
                self.a[x - 1] = y;
                if self.a[x - 1] == 1 {
                    ones.add(x - 1, 1);
                } else if self.a[x - 1] == 2 {
                    twos.add(x - 1, 1);
                } else {
                    threes.add(x - 1, 1);
                }
                let l1 = ones.get(0, z);
                let l2 = twos.get(0, z);
                let l3 = threes.get(0, z);
                let r1 = ones.get(z, self.n);
                let r2 = twos.get(z, self.n);
                let r3 = threes.get(z, self.n);

                let vl = l1 + 2 * l2 + 3 * l3;
                let vr = r1 + 2 * r2 + 3 * r3;
                if vl % 2 != vr % 2 {
                    self.ans -= 1;
                    continue;
                }

                fn solve(
                    mut l1: i32,
                    l2: i32,
                    _l3: i32,
                    _r1: i32,
                    r2: i32,
                    mut r3: i32,
                    mut delta: i32,
                ) -> i32 {
                    let pairs = l1.min(r3).min(delta / 4);
                    delta -= 4 * pairs;
                    l1 -= pairs;
                    r3 -= pairs;
                    let singles23 = l2.min(r3).min(delta / 2);
                    delta -= singles23 * 2;
                    let singles12 = l1.min(r2).min(delta / 2);
                    delta -= singles12 * 2;
                    if delta != 0 {
                        -1
                    } else {
                        pairs + singles23 + singles12
                    }
                }

                if vl < vr {
                    self.ans += solve(l1, l2, l3, r1, r2, r3, vr - vl).into_i64();
                } else {
                    self.ans += solve(r1, r2, r3, l1, l2, l3, vl - vr).into_i64();
                }
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case), self.ans);
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
