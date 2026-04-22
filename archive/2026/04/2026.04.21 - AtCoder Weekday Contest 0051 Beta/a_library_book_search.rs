//{"name":"A - Library Book Search","group":"AtCoder - AtCoder Weekday Contest 0051 Beta","url":"https://atcoder.jp/contests/awc0051/tasks/awc0051_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 2\n3 7 2 5 4\n5 3 7\n1 3\n","output":"21\n"},{"input":"6 4 3\n10 20 30 40 50 60\n25 15 45 35\n2 3 4\n","output":"100\n"},{"input":"10 5 2\n100 200 300 400 500 600 700 800 900 1000\n150 350 550 750 950\n3 5\n","output":"4500\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let p = input.read_size_vec(n).sorted();
    let l = input.read_size_vec(m);
    let t = input.read_size_vec(k).dec();

    let mut max = 0;
    for i in t {
        max.maxim(l[i]);
    }
    let pos = p.upper_bound(&max);
    out.print_line(p.into_iter().take(pos).sum::<usize>());
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
