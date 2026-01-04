//{"name":"coderun6","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _m = input.read_size();
    let xy = input.read_long_pair_vec(n);
    let s = input.read_str();

    let (mut x, mut y) = xy.detuple();
    x.sort();
    y.sort();
    let sx = x.partial_sums();
    let sy = y.partial_sums();
    let mut cx = 0;
    let mut cy = 0;
    for c in s {
        match c {
            b'N' => cy += 1,
            b'S' => cy -= 1,
            b'E' => cx += 1,
            b'W' => cx -= 1,
            _ => unreachable!(),
        }
        let px = x.lower_bound(&cx);
        let py = y.lower_bound(&cy);
        out.print_line(
            cx * px as i64 - sx[px] + (sx[n] - sx[px]) - cx * (n - px) as i64 + cy * py as i64
                - sy[py]
                + (sy[n] - sy[py])
                - cy * (n - py) as i64,
        );
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
