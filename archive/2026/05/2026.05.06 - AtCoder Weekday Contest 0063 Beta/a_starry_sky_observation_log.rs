//{"name":"A - Starry Sky Observation Log","group":"AtCoder - AtCoder Weekday Contest 0063 Beta","url":"https://atcoder.jp/contests/awc0063/tasks/awc0063_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\nT..T.\n.....\n..T..\n","output":"3\n1 1\n1 4\n3 3\n"},{"input":"5 8\n........\n.T....T.\n........\n.T..T...\n....T..T\n","output":"6\n2 2\n2 7\n4 2\n4 5\n5 5\n5 8\n"},{"input":"10 15\nT.............T\n...T...........\n.......T.......\n...............\n.T.............\n...............\n..........T....\n...............\n.............T.\nT..T..T..T..T..\n","output":"12\n1 1\n1 15\n2 4\n3 8\n5 2\n7 11\n9 14\n10 1\n10 4\n10 7\n10 10\n10 13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let s = input.read_char_table(h, w);

    let mut ans = Vec::new();
    for (i, j) in s.indices() {
        if s[(i, j)] == b'T' {
            ans.push((i + 1, j + 1));
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
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
