//{"name":"G. Boring Constructive Problem","group":"Universal Cup - GP of Wulin","url":"https://contest.ucup.ac/contest/3749/problem/18125","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 4 6 8\n2 5 5 9\n3 4 3 7\n","output":"NO\nYES\n2 2 2 2 2\nYES\n3 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut a = input.read_size();
    let mut b = input.read_size();
    let n = input.read_size();
    let s = input.read_size();

    if s % a != 0 || s / a > n {
        out.print_line(true);
        out.print_line(vec![a; n]);
        return;
    }
    if s % b != 0 || s / b > n {
        out.print_line(true);
        out.print_line(vec![b; n]);
        return;
    }
    if a == 2 * b || b == 2 * a || a == s || b == s {
        out.print_line(false);
        return;
    }
    if a > b {
        swap(&mut a, &mut b);
    }
    let len = s / b;
    let ans = Vec::with_gen(n, |i| if i % len == 0 { a } else { b });
    out.print_line(true);
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
