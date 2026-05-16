//{"name":"B - Farm Partitioning","group":"AtCoder - AtCoder Weekday Contest 0068 Beta","url":"https://atcoder.jp/contests/awc0068/tasks/awc0068_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n10 20 30 40 50\n1 2\n3 3\n4 5\n","output":"60\n"},{"input":"8 4\n5 15 10 20 25 5 30 10\n1 3\n4 4\n5 6\n7 8\n","output":"20\n"},{"input":"12 5\n100 200 300 150 250 50 400 100 200 300 150 100\n1 2\n3 5\n6 8\n9 10\n11 12\n","output":"450\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::collections::vec_ext::inc_dec::IncDec;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let lr = input.read_size_pair_vec(m).dec();

    let s = a.partial_sums();
    let mut max = 0;
    let mut min = i64::MAX;
    for (l, r) in lr {
        max.maxim(s[r + 1] - s[l]);
        min.minim(s[r + 1] - s[l]);
    }
    out.print_line(max - min);
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
