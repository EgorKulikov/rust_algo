//{"name":"Corona2020","group":"Eolymp - Basecamp - Educational Round #9","url":"https://eolymp.com/en/compete/1fck3aasuh3ev201dcp2lm8868/problem/2","interactive":false,"timeLimit":1000,"tests":[{"input":"2019 2020 2021\n","output":"2019-2020+2021\n"},{"input":"2019 2020 2022\n","output":"CORONA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let v = input.read_int_vec(3);

    let mut rec = RecursiveFunction2::new(|rec, s: Vec<u8>, val: i32| -> bool {
        if s.len() == 2 {
            if val == 2020 {
                out.print(v[0]);
                for i in 1..3 {
                    out.print(s[i - 1]);
                    out.print(v[i]);
                }
                out.print_line(());
                return true;
            } else {
                return false;
            }
        }
        let mut s1 = s.clone();
        s1.push(b'-');
        if rec.call(s1, val - v[s.len() + 1]) {
            return true;
        }
        let mut s2 = s.clone();
        s2.push(b'+');
        if rec.call(s2, val + v[s.len() + 1]) {
            return true;
        }
        false
    });
    if !rec.call(Vec::new(), v[0]) {
        out.print_line("CORONA");
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
