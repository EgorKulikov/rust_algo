//{"name":"Bounding Robots","group":"Kattis","url":"https://open.kattis.com/problems/boundingrobots","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n1\nu 1\n4 5\n2\nu 3\nr 4\n10 10\n4\nr 30\nd 30\nl 25\nu 5\n0 0\n","output":"Robot thinks 0 1\nActually at 0 1\n\nRobot thinks 4 3\nActually at 3 3\n\nRobot thinks 5 -25\nActually at 0 5\n\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let w = input.read_int();
    let l = input.read_int();

    if w == 0 && l == 0 {
        return;
    }

    let n = input.read_size();
    let moves: Vec<(u8, i32)> = input.read_vec(n);

    let mut x = 0;
    let mut y = 0;
    let mut act_x = 0;
    let mut act_y = 0;
    for (dir, len) in moves {
        match dir {
            b'u' => {
                y += len;
                act_y += len;
            }
            b'd' => {
                y -= len;
                act_y -= len;
            }
            b'l' => {
                x -= len;
                act_x -= len;
            }
            b'r' => {
                x += len;
                act_x += len;
            }
            _ => unreachable!(),
        }
        act_x.maxim(0);
        act_y.maxim(0);
        act_x.minim(w - 1);
        act_y.minim(l - 1);
    }

    output!(out, "Robot thinks {} {}", x, y);
    output!(out, "Actually at {} {}", act_x, act_y);
    out.print_line(());
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
