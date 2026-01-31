#![allow(unexpected_cfgs)]
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
            if s.len() != 1 {
                loop {}
            }
            if !val.contains_key(&s[0]) {
                loop {}
            }
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
mod tester {
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{run, TASK_TYPE, TEST_TYPE, TestType};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::Random;
use algo_lib::string::str::StrReader;
use std::io::{Read, Write};
use std::thread::yield_now;
use tester::classic::default_checker;
use tester::classic::EPS;
use tester::interactive::std_interactor;
use tester::interactive::SolutionRunner;
use tester::test_set::GeneratedTestSet;
use tester::Tester;

const PRINT_LIMIT: usize = 1000;

fn interact(mut input: Input, expected: Option<Input>, mut runner: SolutionRunner) -> Result<(), String> {
    let (mut sol, mut out) = runner.run();
    Ok(())
}

fn run_twice(
    mut input: Input,
    expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<(), String> {
    let (mut sol, mut out) = runner.run();
    input.read_line();
    out.print_line("first");
    let t = match TEST_TYPE {
        TestType::RunTwiceSingle => None,
        TestType::RunTwiceMultiNumber => {
            let t = input.read_size();
            out.print_line(t);
            Some(t)
        }
        _ => unreachable!(),
    };
    let mut input_vec = Vec::new();
    input.read_to_end(&mut input_vec).unwrap();
    out.write_all(&input_vec).unwrap();
    out.flush();
    while !runner.is_finished() {
        yield_now();
    }
    let mut first_output = Vec::new();
    sol.read_to_end(&mut first_output).unwrap();

    let (mut sol, mut out) = runner.run();
    out.print_line("second");
    if let Some(t) = t {
        out.print_line(t);
    }
    out.write_all(&first_output).unwrap();
    out.flush();
    let mut ans = Vec::new();
    sol.read_to_end(&mut ans).unwrap();
    default_checker(Input::slice(&input_vec), expected, Input::slice(&ans))
    // check(Input::slice(&input_vec), expected, Input::slice(&ans))
}

fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
    Ok(())
}

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item=Self::TestId> {
        1..
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new();
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

struct MaxTest;

impl GeneratedTestSet for MaxTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item=Self::TestId> {
        1..=1
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new_with_seed(239);
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./t727431_202601_";
    let tl = 1000;
    let tester = match TASK_TYPE {
        crate::TaskType::Interactive => {
            Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, run_twice)
        }
        crate::TaskType::Classic => {
            Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
            // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
        }
    };
    let passed = tester.test_samples();
    // tester.test_generated("Max test", true, MaxTest);
    // tester.test_generated("Stress test", false, StressTest);
    passed
}
}
#[test]
fn t727431_202601_() {
    assert!(tester::run_tests());
}
