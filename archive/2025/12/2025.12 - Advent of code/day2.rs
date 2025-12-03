//{"name":"day_2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::str_scan;
use algo_lib::string::split::StrSplit;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let tokens = s.str_split(b",");

    let mut ans = 0;
    let mut ans2 = 0;
    for token in tokens {
        str_scan!(token, "@-@", from: usize, to: usize);
        for id in from..=to {
            let s = id.to_string();
            let s = Str::from(s.as_bytes());
            if s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..] {
                ans += id;
            }
            for i in 2..=s.len() {
                if s.len() % i != 0 {
                    continue;
                }
                let part_len = s.len() / i;
                let mut ok = true;
                for j in (part_len..s.len()).step_by(part_len) {
                    if s[..part_len] != s[j..j + part_len] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ans2 += id;
                    break;
                }
            }
        }
    }
    out.print_line(ans);
    out.print_line(ans2);
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
