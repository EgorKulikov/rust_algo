//{"name":"A Rational Sequence","group":"Kattis","url":"https://open.kattis.com/problems/rationalsequence","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 1/1\n2 1/3\n3 5/2\n4 2178309/1346269\n5 1/10000000\n","output":"1 1/2\n2 3/2\n3 2/5\n4 1346269/1860498\n5 10000000/9999999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::scan;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    assert_eq!(test_case, input.read_size());
    scan!(input, "@/@", p: usize, q: usize);

    if q == 1 {
        writeln!(out, "{test_case} 1/{}", p + 1).unwrap();
        return;
    }
    let mut p = p;
    let mut q = q;
    let times = p / q;
    p -= times * q;
    q -= p;
    p += q;
    q += times * p;
    writeln!(out, "{test_case} {}/{}", p, q).unwrap();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
