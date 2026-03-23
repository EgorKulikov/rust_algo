//{"name":"A - Reversi 3","group":"AtCoder - AtCoder Regular Contest++ 216","url":"https://atcoder.jp/contests/arc216/tasks/arc216_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n0001\n0111\n6\n101101\n011100\n5\n10101\n10101\n10\n0101000101\n0011100111\n","output":"2\n-1\n0\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_str();
    let b = input.read_str();

    if a[0] != b[0] {
        out.print_line(-1);
        return;
    }

    #[derive(Default)]
    struct Node {
        delta: i32,
    }
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            self.delta ^= value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node {
        delta: a[i] as i32 - b'0' as i32,
    });
    let mut ans = 0;
    let mut at = 0;
    for i in 1..n {
        if (st.point_query(i).delta == 0) != (b[i] == b'0') {
            at.maxim(i + 1);
            loop {
                if at == n {
                    out.print_line(-1);
                    return;
                }
                if st.point_query(at - 2).delta == st.point_query(at).delta {
                    break;
                }
                at += 1;
            }
            st.update(i..at, &Node { delta: 1 });
            ans += at - i;
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
