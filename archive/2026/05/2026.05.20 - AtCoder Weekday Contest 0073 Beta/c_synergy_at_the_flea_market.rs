//{"name":"C - Synergy at the Flea Market","group":"AtCoder - AtCoder Weekday Contest 0073 Beta","url":"https://atcoder.jp/contests/awc0073/tasks/awc0073_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 2\n3 3\n7 4\n","output":"6\n"},{"input":"4 10\n1 1\n2 2\n5 3\n8 4\n","output":"35\n"},{"input":"8 5\n0 3\n2 7\n4 1\n8 5\n10 2\n15 6\n17 4\n20 8\n","output":"162\n"},{"input":"15 100\n10 50\n200 30\n350 80\n500 20\n610 90\n750 40\n880 60\n1000 10\n1050 70\n1200 100\n1350 25\n1500 55\n1600 35\n1700 45\n1800 15\n","output":"4875\n"},{"input":"1 1000000000\n500000000 10000\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_long();
    let xs = input.read_long_pair_vec(n).sorted();

    let mut at = 0;
    let mut ans = 0;
    let mut sum = 0;
    for (x, c) in xs.copy_iter() {
        while xs[at].0 < x - d {
            sum -= xs[at].1;
            at += 1;
        }
        ans += sum * c;
        sum += c;
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
