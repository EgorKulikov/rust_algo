//{"name":"Arithmetic mean for figure skaters","group":"Eolymp - Basecamp - Educational Round #7","url":"https://eolymp.com/en/compete/0fdopgb6et7g3allmrhbicn9cg/problem/2","interactive":false,"timeLimit":1000,"tests":[{"input":"5 4\n7 8 9 8 10\n6 5 5 4 7\n9 9 10 7 7\n7 7 10 9 8\n","output":"8.33 5.33 9.00 8.50\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut ans = Vec::with_capacity(m);
    for _ in 0..m {
        let v = input.read_real_vec(n);
        let max = v.copy_max();
        let min = v.copy_min();
        let mut sum = Real(0.);
        let mut qty = 0;
        for x in v {
            if x != max && x != min {
                sum += x;
                qty += 1;
            }
        }
        ans.push(sum / qty);
    }
    out.set_precision(2);
    out.print_line(ans);
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
