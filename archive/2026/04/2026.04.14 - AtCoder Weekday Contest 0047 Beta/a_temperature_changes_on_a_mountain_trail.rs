//{"name":"A - Temperature Changes on a Mountain Trail","group":"AtCoder - AtCoder Weekday Contest 0047 Beta","url":"https://atcoder.jp/contests/awc0047/tasks/awc0047_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n10\n12\n15\n14\n20\n","output":"2\n"},{"input":"4 10\n5\n8\n3\n12\n","output":"0\n"},{"input":"10 5\n0\n-4\n2\n7\n1\n10\n6\n-1\n4\n4\n","output":"6\n"},{"input":"20 100\n0\n50\n-60\n40\n140\n139\n239\n100\n0\n-100\n-50\n49\n149\n300\n201\n101\n1\n-98\n-198\n-97\n","output":"13\n"},{"input":"2 1000000000\n-1000000000\n0\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let t = input.read_int_vec(n);

    out.print_line(
        t.consecutive_iter_copy()
            .filter(|(a, b)| (a - b).abs() >= k)
            .count(),
    );
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
