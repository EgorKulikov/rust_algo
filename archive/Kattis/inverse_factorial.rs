//{"name":"Inverse Factorial","group":"Kattis","url":"https://open.kattis.com/problems/inversefactorial","interactive":false,"timeLimit":1000,"tests":[{"input":"120\n","output":"5\n"},{"input":"51090942171709440000\n","output":"21\n"},{"input":"10888869450418352160768000000\n","output":"27\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::io::scan::Parse;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let nf = input.read_str();

    let prefix: i64 = nf[..nf.len().min(12)].parse();
    let suffix = nf.len().saturating_sub(12);
    let mut x = 1i64;
    let mut y = 0;
    let mut ans = None;
    for i in 1..=1000000i64 {
        x *= i;
        while x >= 1_000_000_000_000 {
            x /= 10;
            y += 1;
        }
        if y == suffix {
            ans.minim(((x - prefix).abs(), i));
        } else if y + 1 == suffix {
            ans.minim(((x * 10 - prefix).abs(), i));
        }
    }
    out.print_line(ans.unwrap().1);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
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
//END MAIN
