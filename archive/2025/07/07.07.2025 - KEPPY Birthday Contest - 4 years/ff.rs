//{"name":"ff","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_4d::Memoization4d;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let n = s.len();
    let mut mem = Memoization4d::new(n + 1, 2, 3, 3, |mem, pos, used, wins, losses| -> i32 {
        if pos == n {
            0
        } else {
            let score = |c: u8| -> (i32, usize, usize) {
                match c {
                    b'W' => (if wins < 2 { 2 } else { 3 }, (wins + 1).min(2), 0),
                    b'D' => (1, 0, 0),
                    b'L' => (if losses < 2 { 0 } else { -1 }, 0, (losses + 1).min(2)),
                    b'S' => (5, 0, 0),
                    _ => unreachable!(),
                }
            };
            let (pts, new_wins, new_losses) = score(s[pos]);
            let mut res = pts + mem.call(pos + 1, used, new_wins, new_losses);
            if used == 0 {
                for &c in b"WS" {
                    let (pts, new_wins, new_losses) = score(c);
                    res.maxim(pts + mem.call(pos + 1, 1, new_wins, new_losses));
                }
            }
            res
        }
    });
    out.print_line(mem.call(0, 0, 0, 0));
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
