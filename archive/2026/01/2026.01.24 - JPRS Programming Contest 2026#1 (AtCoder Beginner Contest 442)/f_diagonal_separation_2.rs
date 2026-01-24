//{"name":"F - Diagonal Separation 2","group":"AtCoder - JPRS Programming Contest 2026#1 (AtCoder Beginner Contest 442)","url":"https://atcoder.jp/contests/abc442/tasks/abc442_f","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n..#\n#.#\n.#.\n","output":"2\n"},{"input":"5\n..#.#\n#..##\n###.#\n.###.\n#....\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_char_table(n, n);

    let mut black_left = Arr2d::new(n, n + 1, 0);
    for (i, j) in s.indices() {
        black_left[(i, j + 1)] = black_left[(i, j)] + if s[(i, j)] == b'#' { 1 } else { 0 };
    }
    let mut white_right = Arr2d::new(n, n + 1, 0);
    for (i, j) in s.indices().rev() {
        white_right[(i, j)] = white_right[(i, j + 1)] + if s[(i, j)] == b'.' { 1 } else { 0 };
    }
    let mut mem = Memoization2d::new(n + 1, n + 1, |mem, row, col| -> i32 {
        if row == 0 {
            0
        } else {
            let mut res =
                mem.call(row - 1, col) + black_left[(row - 1, col)] + white_right[(row - 1, col)];
            if col != n {
                res.minim(mem.call(row, col + 1));
            }
            res
        }
    });
    out.print_line(mem.call(n, 0));
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
