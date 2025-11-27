//{"name":"Jim and the Skyscrapers","group":"Eolymp - Basecamp - Educational Round #5","url":"https://eolymp.com/en/compete/saaq21a9v514tbj0spj7bmgjpk/problem/7","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3 2 1 2 3 3\n","output":"4\n"},{"input":"10\n5 3 5 3 2 3 1 3 5 5\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_size_vec(n);

    let mut s = Vec::new();
    let mut ans = 0usize;
    for i in h {
        while let Some(&(last, qty)) = s.last() {
            if last < i {
                s.pop();
            } else if last == i {
                ans += qty;
                break;
            } else {
                break;
            }
        }
        if let Some((last, qty)) = s.last_mut() {
            if *last == i {
                *qty += 1;
            } else {
                s.push((i, 1));
            }
        } else {
            s.push((i, 1));
        }
    }
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
