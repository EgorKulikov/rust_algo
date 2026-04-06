//{"name":"A - Password Verification","group":"AtCoder - AtCoder Weekday Contest 0039 Beta","url":"https://atcoder.jp/contests/awc0039/tasks/awc0039_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2 3\n1 a\n4 d\nabcde\nxbcde\nabcdz\n","output":"Yes\nNo\nYes\n"},{"input":"3 0 2\nabc\nxyz\n","output":"Yes\nYes\n"},{"input":"10 4 5\n2 b\n5 e\n7 g\n10 z\nabcdefghiz\nxbxdexgxxz\nabcdeagxyz\nxbxdexhxxz\nabcdefghij\n","output":"Yes\nYes\nYes\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let pc = input.read_vec::<(usize, u8)>(m);

    for _ in 0..q {
        let s = input.read_str();
        out.print_line(pc.copy_all(|(p, c)| s[p - 1] == c));
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
