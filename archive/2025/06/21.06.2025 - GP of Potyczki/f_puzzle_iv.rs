//{"name":"F. Puzzle IV","group":"Universal Cup - GP of Potyczki","url":"https://contest.ucup.ac/contest/2135/problem/12155","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 3 2\n","output":"3\n2 - 3\n3 + 2\n2 + 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n);

    struct State {
        p: Vec<usize>,
        order: BTreeSet<(usize, usize)>,
        ans: Vec<(usize, u8, usize)>,
    }

    impl State {
        fn new(p: Vec<usize>) -> Self {
            let mut order = BTreeSet::new();
            for i in 0..p.len() {
                order.insert((p[i], i));
            }
            Self {
                p,
                order,
                ans: Vec::new(),
            }
        }

        fn get(&self) -> Option<usize> {
            for &(_, i) in self.order.iter().rev() {
                if i > 0 && self.p[i - 1] < self.p[i]
                    || i + 1 < self.p.len() && self.p[i] > self.p[i + 1]
                {
                    return Some(i);
                }
            }
            None
        }

        fn add(&mut self, i: usize, j: usize) {
            self.order.remove(&(self.p[i], i));
            assert!(self.p[i] + self.p[j] <= self.p.len());
            assert!(i.abs_diff(j) == 1);
            self.p[i] += self.p[j];
            self.order.insert((self.p[i], i));
            self.ans.push((i + 1, b'+', j + 1));
        }

        fn sub(&mut self, i: usize, j: usize) {
            self.order.remove(&(self.p[i], i));
            assert!(self.p[i] > self.p[j]);
            assert!(i.abs_diff(j) == 1);
            self.p[i] -= self.p[j];
            self.order.insert((self.p[i], i));
            self.ans.push((i + 1, b'-', j + 1));
        }

        fn check(&self) {
            assert_eq!(self.p, (1..=self.p.len()).collect::<Vec<_>>());
        }
    }

    let mut s = State::new(p);
    while let Some(max_pos) = s.get() {
        let other = if max_pos == 0 {
            1
        } else if max_pos == n - 1 {
            n - 2
        } else if s.p[max_pos - 1] < s.p[max_pos + 1] && s.p[max_pos + 1] != s.p[max_pos]
            || s.p[max_pos - 1] == s.p[max_pos]
        {
            max_pos + 1
        } else {
            max_pos - 1
        };
        loop {
            if other != 0 && other != n - 1 {
                let side = 2 * other - max_pos;
                if s.p[side] > s.p[other] && s.p[side] + s.p[other] < s.p[max_pos] {
                    s.add(other, side);
                } else if s.p[side] + 2 * s.p[other] < s.p[max_pos] {
                    s.add(side, other);
                    s.add(other, side);
                } else if 3 * s.p[other] < s.p[max_pos] {
                    if s.p[side] == s.p[max_pos] {
                        s.sub(side, other);
                        s.sub(side, other);
                        s.add(other, side);
                    } else {
                        s.sub(side, other);
                        s.add(other, side);
                    }
                    break;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        s.sub(max_pos, other);
    }
    for i in 1..n {
        s.add(i, i - 1);
    }
    s.check();
    let ans = s.ans;
    out.print_line(ans.len());
    for (i, op, j) in ans {
        out.print_line((i, op, j));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
