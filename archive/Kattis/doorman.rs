//{"name":"Doorman","group":"Kattis","url":"https://open.kattis.com/problems/doorman","interactive":false,"timeLimit":1000,"tests":[{"input":"1\nMWWMWMMWM\n","output":"9\n"},{"input":"2\nWMMMMWWMMMWWMW\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_int();
    let s = input.read_str();

    let mut ans = 0;
    let mut skipped = false;
    let mut delta = 0;
    for c in s {
        match c {
            b'M' => {
                if delta == x {
                    if skipped {
                        break;
                    }
                    skipped = true;
                } else if delta == -x {
                    ans += 1;
                    if skipped {
                        skipped = false;
                        ans += 1;
                    } else {
                        delta += 1;
                    }
                } else {
                    ans += 1;
                    delta += 1;
                }
            }
            b'W' => {
                if delta == -x {
                    if skipped {
                        break;
                    }
                    skipped = true;
                } else if delta == x {
                    ans += 1;
                    if skipped {
                        skipped = false;
                        ans += 1;
                    } else {
                        delta -= 1;
                    }
                } else {
                    ans += 1;
                    delta -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
    out.print_line(ans);
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
