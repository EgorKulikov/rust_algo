//{"name":"J3 - Product Codes","group":"DMOJ - CCC '25","url":"https://dmoj.ca/problem/ccc25j3","interactive":false,"timeLimit":1000,"tests":[{"input":"1\nAbC3c2Cd9\n","output":"ACC14\n"},{"input":"3\nAhkiy-6ebvXCV1\n393hhhUHkbs5gh6QpS-9-8\nPL12N-2G1234Duytrty8-86tyaYySsDdEe\n","output":"AXCV-5\nUHQS387\nPLNGDYSDE1166\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use algo_lib::{output, when};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let mut ans = Str::new();
    let mut sign = 0;
    let mut val = 0;
    let mut sum = 0;
    for c in s {
        when! {
            c == b'-' => {
                sum += sign * val;
                sign = -1;
                val = 0;
            },
            c.is_ascii_digit() => {
                if sign == 0 {
                    sign = 1;
                    val = 0;
                }
                val *= 10;
                val += (c - b'0') as i64;
            },
            c.is_ascii_uppercase() => {
                sum += sign * val;
                sign = 0;
                ans.push(c);
            },
            c.is_ascii_lowercase() => {
                sum += sign * val;
                sign = 0;
            },
            else => unreachable!(),
        }
    }
    sum += sign * val;
    output!(out, "{}{}", ans, sum);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
