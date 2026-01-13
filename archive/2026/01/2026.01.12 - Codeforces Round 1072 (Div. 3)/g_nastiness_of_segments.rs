//{"name":"G. Nastiness of Segments","group":"Codeforces - Codeforces Round 1072 (Div. 3)","url":"https://codeforces.com/contest/2184/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"1\n5 5\n1 2 3 4 5\n2 1 5\n1 1 5\n1 2 5\n1 3 1\n2 1 5\n","output":"1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    #[derive(Default)]
    struct Node {
        to: usize,
        min: usize,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.to = right_val.to;
            self.min = left_val.min.min(right_val.min);
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node { to: i, min: a[i] });
    for _ in 0..q {
        let mode = input.read_int();
        match mode {
            1 => {
                let i = input.read_size() - 1;
                let x = input.read_size();
                st.point_update(i, Node { to: i, min: x });
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let mut res = 0;
                let mut min = usize::MAX;
                st.binary_search_in(
                    l..r,
                    |node| {
                        let len = node.to - l;
                        let val = min.min(node.min);
                        if len == val {
                            res = 1;
                        }
                        if len < val {
                            min.minim(node.min);
                            false
                        } else {
                            true
                        }
                    },
                    |_, _| (),
                );
                out.print_line(res);
            }
            _ => unreachable!(),
        }
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
