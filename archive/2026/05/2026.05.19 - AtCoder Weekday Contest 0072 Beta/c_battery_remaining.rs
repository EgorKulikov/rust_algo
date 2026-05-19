//{"name":"C - Battery Remaining","group":"AtCoder - AtCoder Weekday Contest 0072 Beta","url":"https://atcoder.jp/contests/awc0072/tasks/awc0072_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n10 5 3 8 2\n1 3\n2 4\n3 5\n","output":"9 3 0 6 1\n"},{"input":"4 5\n3 1 4 2\n1 2\n1 2\n2 3\n3 4\n1 4\n","output":"0 0 1 0\n"},{"input":"10 8\n100 0 5 20 3 15 8 0 12 7\n1 5\n3 7\n2 6\n5 10\n1 10\n6 8\n4 4\n1 3\n","output":"97 0 0 15 0 10 4 0 10 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_long_vec(n);
    let lr = input.read_size_pair_vec(q).dec();

    let mut delta = vec![0; n + 1];
    for (l, r) in lr {
        delta[l] += 1;
        delta[r + 1] -= 1;
    }
    let mut cur = 0;
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        cur += delta[i];
        ans.push((s[i] - cur).max(0));
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
