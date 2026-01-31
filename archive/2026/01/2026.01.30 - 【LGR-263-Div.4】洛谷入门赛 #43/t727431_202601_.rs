//{"name":"T727431 [语言月赛 202601] 方程检查","group":"Luogu","url":"https://www.luogu.com.cn/problem/T727431?contestId=304401","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\nx+y=25\nx=5\ny=5\n3\na+b=b-c-c\na=2\nb=1\nc=-1\n","output":"No\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::io::scan::Parse;
use algo_lib::misc::expression_parser::ExpressionParser;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::scan;
use algo_lib::string::str::Str;
use std::ops::{Add, Neg, Sub};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    scan!(input, "@=@", left: Str, right: Str);
    let mut val = FxHashMap::default();
    for _ in 0..n {
        scan!(input, "@=@", var: u8, value: i64);
        val.insert(var, value);
    }

    let mut parser = ExpressionParser::new(|s| {
        if s[0].is_ascii_digit() {
            s.parse()
        } else {
            val[&s[0]]
        }
    });
    parser.add_binary_op(0, b'+', i64::add);
    parser.add_binary_op(0, b'-', i64::sub);
    parser.add_unary_op(b'-', i64::neg);
    let left_val = parser.parse(&left);
    let right_val = parser.parse(&right);
    out.print_line(left_val == right_val);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
