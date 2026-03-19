//{"name":"M. May We Answer Your Questions Right?","group":"Universal Cup - GP of Belarus","url":"https://contest.ucup.ac/contest/3426/problem/17272","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4 7\n9 -4 2 -1\n2 1 4\n2 2 3\n2 2 4\n1 1 8\n2 1 4\n1 2 -5\n2 1 4\n7 1\n2026 2 5 -14 59 59 -75\n2 1 7\n","output":"1\n0\n-1\n0\n-1\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Ordering;
use std::num::Saturating;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    #[derive(Default, Clone)]
    struct Node {
        k: i64,
        delta: Saturating<i64>,
        len: usize,
    }

    const LIMIT: i64 = 1_000_000_001;

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.len = left_val.len + right_val.len;
            if right_val.delta.0 > LIMIT || right_val.delta.0 < -LIMIT {
                self.k = right_val.k;
                self.delta = right_val.delta;
                return;
            }
            let mut k = right_val.k;
            let mut mid = left_val.k + right_val.delta.0;
            if right_val.len <= 30 {
                let step = 1 << right_val.len;
                let shift = mid / step;
                k += shift;
                mid -= shift * step;
                while mid * 2 > step {
                    k += 1;
                    mid -= step;
                }
                while mid * 2 <= -step {
                    k -= 1;
                    mid += step;
                }
            }
            let delta = if mid == 0 {
                left_val.delta
            } else {
                let mut delta = Saturating(mid);
                for _ in 0..left_val.len {
                    delta *= 2;
                    if delta.0 > LIMIT || delta.0 < -LIMIT {
                        break;
                    }
                }
                if delta.0.abs() <= LIMIT {
                    delta += left_val.delta;
                }
                delta
            };
            self.k = k;
            self.delta = delta;
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node {
        k: a[i] / 2,
        delta: Saturating(a[i] % 2),
        len: 1,
    });

    for _ in 0..q {
        let command = input.read_int();
        match command {
            1 => {
                let pos = input.read_size() - 1;
                let val = input.read_long();
                st.point_update(
                    pos,
                    Node {
                        k: val / 2,
                        delta: Saturating(val % 2),
                        len: 1,
                    },
                );
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let res = st.query(l..r);
                match res.k.cmp(&0) {
                    Ordering::Less => out.print_line(-1),
                    Ordering::Equal => match res.delta.0.cmp(&0) {
                        Ordering::Less => out.print_line(-1),
                        Ordering::Equal => out.print_line(0),
                        Ordering::Greater => out.print_line(1),
                    },
                    Ordering::Greater => out.print_line(1),
                }
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
