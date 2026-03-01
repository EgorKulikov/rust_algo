//{"name":"T736382 数学补习","group":"Luogu","url":"https://www.luogu.com.cn/problem/T736382?contestId=311027","interactive":false,"timeLimit":1000,"tests":[{"input":"36 12 2\n","output":"9\n"},{"input":"11 18 3\n","output":"13\n"},{"input":"13 22 2\n","output":"5\n"},{"input":"7 10 10\n","output":"63\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_int();
    let y = input.read_int();
    let z = input.read_int();

    let mut ans = 0;
    for a in 10..100 {
        for b in 10..100 {
            let mut cx = a;
            let mut cy = b;
            for c in 0..z {
                if (cx + cy) % 2 == 1 {
                    cx -= cy % cx;
                } else {
                    cy -= cx % cy;
                }
                if cx < c {
                    cx += cy / 2 + 1;
                }
                if cy < c {
                    cy += cx / 2 + 1;
                }
            }
            if cx == x && cy == y {
                ans += 1;
            }
        }
    }
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
