//{"name":"Treasure Hunt","group":"Kattis","url":"https://open.kattis.com/problems/treasurehunt","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2\nES\nTW\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let r = input.read_size();
    let c = input.read_size();
    let grid = input.read_char_table(r, c);

    let mut row = 0;
    let mut col = 0;
    let mut visited = Arr2d::new(r, c, false);
    for i in 0.. {
        if row < 0 || row >= r as isize || col < 0 || col >= c as isize {
            out.print_line("Out");
            return;
        }
        if visited[(row, col)] {
            out.print_line("Lost");
            return;
        }
        visited[(row, col)] = true;
        match grid[(row, col)] {
            b'N' => row -= 1,
            b'S' => row += 1,
            b'E' => col += 1,
            b'W' => col -= 1,
            b'T' => {
                out.print_line(i);
                return;
            }
            _ => unreachable!(),
        }
    }
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

//START MAIN
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
//END MAIN
