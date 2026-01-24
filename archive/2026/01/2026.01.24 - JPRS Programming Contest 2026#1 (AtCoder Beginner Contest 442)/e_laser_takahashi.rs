//{"name":"E - Laser Takahashi","group":"AtCoder - JPRS Programming Contest 2026#1 (AtCoder Beginner Contest 442)","url":"https://atcoder.jp/contests/abc442/tasks/abc442_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n0 1\n1 -2\n1 0\n-2 0\n3 0\n4 1\n1 4\n5 4\n3 5\n","output":"2\n5\n4\n2\n"},{"input":"2 1\n1 2\n1 2\n1 2\n","output":"2\n"},{"input":"8 10\n-84 -60\n-100 8\n77 55\n-14 -10\n50 -4\n-63 -45\n26 -17\n-7 -5\n3 7\n2 4\n8 4\n8 4\n7 1\n1 7\n6 3\n4 7\n4 5\n2 6\n","output":"3\n8\n4\n4\n5\n8\n6\n8\n7\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::rational::Rational;
use algo_lib::when;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let pts = input.read_long_pair_vec(n);

    let val = Vec::with_gen(n, |i| {
        let (x, y) = pts[i];
        when! {
            x > 0 && y >= 0 => {
                (3, -Rational::new(y, x))
            },
            y > 0 && x <= 0 => {
                (2, -Rational::new(-x, y))
            },
            x < 0 && y <= 0 => {
                (1, -Rational::new(-y, -x))
            },
            y < 0 && x >= 0 => {
                (0, -Rational::new(x, -y))
            },
        }
    });
    let order = val.clone().sorted();

    for _ in 0..q {
        let a = input.read_size() - 1;
        let b = input.read_size() - 1;
        let from = order.lower_bound(&val[a]);
        let to = order.upper_bound(&val[b]);
        if from < to {
            out.print_line(to - from);
        } else {
            out.print_line(n + to - from);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
