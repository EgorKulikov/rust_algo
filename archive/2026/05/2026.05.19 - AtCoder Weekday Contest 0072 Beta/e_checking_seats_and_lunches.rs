//{"name":"E - Checking Seats and Lunches","group":"AtCoder - AtCoder Weekday Contest 0072 Beta","url":"https://atcoder.jp/contests/awc0072/tasks/awc0072_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n10 3 5 8 2\n4 3 6 7 1\n1 5\n2 4\n3 3\n","output":"No\nNo\nNo\n"},{"input":"4 5\n5 0 3 10\n5 1 3 10\n1 1\n1 4\n2 2\n2 3\n4 4\n","output":"Yes\nNo\nNo\nNo\nYes\n"},{"input":"10 8\n100 50 30 20 60 10 45 80 25 70\n90 50 35 15 60 10 40 85 20 65\n1 10\n1 5\n3 3\n4 6\n6 6\n7 9\n5 8\n1 3\n","output":"No\nNo\nNo\nYes\nYes\nNo\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let m = input.read_size();
    let s = input.read_int_vec(n);
    let p = input.read_int_vec(n);

    let g = Vec::with_gen(n, |i| (s[i] >= p[i]) as usize);
    let sg = g.partial_sums();
    for _ in 0..m {
        let l = input.read_size() - 1;
        let r = input.read_size();
        out.print_line(sg[r] - sg[l] == r - l);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
