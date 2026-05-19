//{"name":"B - Tending the Flower Bed","group":"AtCoder - AtCoder Weekday Contest 0072 Beta","url":"https://atcoder.jp/contests/awc0072/tasks/awc0072_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n3 -5 4 2 -1\n","output":"9\n"},{"input":"4 3\n-2 -7 -1 -3\n","output":"-10\n"},{"input":"10 4\n5 -2 3 -1 6 -7 4 2 -3 8\n","output":"25\n"},{"input":"20 6\n4 -5 7 0 -2 9 -8 3 1 -4 6 -1 5 -7 2 8 -3 4 -6 10\n","output":"54\n"},{"input":"1 1\n-1000000000\n","output":"-1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);

    let s = a.partial_sums();
    let b = Vec::with_gen(n, |i| a[i].max(0));
    let t = b.partial_sums();
    let mut ans = None;
    for i in 0..=n - k {
        ans.maxim(t[i] + s[i + k] - s[i] + t[n] - t[i + k]);
    }
    out.print_line(ans);
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
