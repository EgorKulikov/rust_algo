//{"name":"C - Stacking Cards","group":"AtCoder - AtCoder Weekday Contest 0050 Beta","url":"https://atcoder.jp/contests/awc0050/tasks/awc0050_c","interactive":false,"timeLimit":2000,"tests":[{"input":"7\nPUT 3\nPUT 5\nLOOK\nPUT 6\nLOOK\nREMOVE\nLOOK\n","output":"-1\n-1\n1\n"},{"input":"6\nPUT 1\nLOOK\nPUT 2\nLOOK\nPUT 4\nLOOK\n","output":"-1\n-1\n-1\n"},{"input":"15\nPUT 10\nPUT 7\nLOOK\nREMOVE\nLOOK\nPUT 3\nLOOK\nPUT 9\nLOOK\nREMOVE\nREMOVE\nLOOK\nPUT 5\nLOOK\nREMOVE\n","output":"-1\n-1\n-1\n-1\n2\n-1\n"},{"input":"24\nPUT 1\nPUT 2\nPUT 4\nLOOK\nREMOVE\nLOOK\nREMOVE\nLOOK\nPUT 8\nLOOK\nPUT 6\nLOOK\nREMOVE\nLOOK\nREMOVE\nLOOK\nPUT 3\nLOOK\nPUT 2\nLOOK\nREMOVE\nLOOK\nREMOVE\nLOOK\n","output":"-1\n-1\n-1\n-1\n-1\n4\n3\n-1\n-1\n8\n7\n"},{"input":"2\nPUT 1000000000\nLOOK\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut xor = 0;
    let mut stack = Vec::new();
    let mut last = FxHashMap::default();
    let mut id = 1;
    for _ in 0..n {
        let mode = input.read_str();
        match mode.as_slice() {
            b"PUT" => {
                let x = input.read_size();
                stack.push(x);
                xor ^= x;
            }
            b"REMOVE" => {
                if let Some(x) = stack.pop() {
                    xor ^= x;
                }
            }
            b"LOOK" => {
                out.print_line(last.get(&xor));
                last.insert(xor, id);
                id += 1;
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
