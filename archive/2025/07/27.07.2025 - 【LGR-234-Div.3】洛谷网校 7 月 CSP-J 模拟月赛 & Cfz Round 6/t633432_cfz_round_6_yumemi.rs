//{"name":"T633432 「Cfz Round 6」Yumemi","group":"Luogu","url":"https://www.luogu.com.cn/problem/T633432?contestId=255793","interactive":false,"timeLimit":1000,"tests":[{"input":"4 8\n15\n24\n37\n80\n","output":"kawaii\nkawaii\ndame\nkawaii\n"},{"input":"3 998244353\n31415926535\n9999999999\n17320508075\n","output":"kawaii\ndame\nkawaii\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();

    'outer: for _ in 0..n {
        let s = input.read_str();
        let mut rem = k;
        for c in s {
            if c == b'0' {
                out.print_line("kawaii");
                continue 'outer;
            }
            rem /= (c - b'0') as i64;
        }
        if rem != 0 {
            out.print_line("kawaii");
        } else {
            out.print_line("dame");
        }
    }
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
