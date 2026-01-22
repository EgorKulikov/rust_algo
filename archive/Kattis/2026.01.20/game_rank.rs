//{"name":"Game Rank","group":"Kattis","url":"https://open.kattis.com/problems/gamerank","interactive":false,"timeLimit":1000,"tests":[{"input":"WW\n","output":"25\n"},{"input":"WWW\n","output":"24\n"},{"input":"WWWW\n","output":"23\n"},{"input":"WLWLWLWL\n","output":"24\n"},{"input":"WWWWWWWWWLLWW\n","output":"19\n"},{"input":"WWWWWWWWWLWWL\n","output":"18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let mut rank = 25;
    let mut stars = 0;
    let mut wins = 0;
    fn stars_for_rank(r: i32) -> i32 {
        if r == 0 {
            i32::MAX
        } else {
            (6 - (r - 1) / 5).min(5)
        }
    }
    for c in s {
        match c {
            b'W' => {
                wins += 1;
                stars += 1;
                if wins >= 3 && rank >= 6 {
                    stars += 1;
                }
                let need = stars_for_rank(rank);
                if stars > need {
                    stars -= need;
                    rank -= 1;
                }
            }
            b'L' => {
                wins = 0;
                if rank <= 20 && rank > 0 {
                    stars -= 1;
                    if stars < 0 && rank < 20 {
                        rank += 1;
                        stars = stars_for_rank(rank) - 1;
                    } else {
                        stars.maxim(0);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    if rank == 0 {
        out.print_line("Legend");
    } else {
        out.print_line(rank);
    }
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
