//{"name":"day11","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::eol::EolVec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::{Memoization, Memoization3};
use algo_lib::misc::recursive_function::{Callable, Callable3};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::scan;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut edges = DefaultHashMap::new(Vec::new());
    while !input.is_empty() {
        scan!(input, "@: @", label: Str, neighbors: EolVec<Str>);
        edges[label] = neighbors.unwrap();
    }

    let mut mem = Memoization::new(|mem, cur: &Str| -> i64 {
        if cur.as_slice() == b"out" {
            1
        } else {
            edges[cur.clone()].iter().map(|nxt| mem.call(nxt)).sum()
        }
    });
    let ans = mem.call(&Str::from(b"you"));
    out.print_line(ans);
    let mut mem = Memoization3::new(
        |mem, cur: &Str, dac_visited: bool, fft_visited: bool| -> i64 {
            if cur.as_slice() == b"out" {
                if dac_visited && fft_visited {
                    1
                } else {
                    0
                }
            } else {
                edges[cur.clone()]
                    .iter()
                    .map(|nxt| {
                        mem.call(
                            nxt,
                            dac_visited || nxt.as_slice() == b"dac",
                            fft_visited || nxt.as_slice() == b"fft",
                        )
                    })
                    .sum()
            }
        },
    );
    let ans = mem.call(&Str::from(b"svr"), false, false);
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
