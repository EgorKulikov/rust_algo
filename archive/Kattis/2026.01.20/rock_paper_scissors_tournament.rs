//{"name":"Rock-Paper-Scissors Tournament","group":"Kattis","url":"https://open.kattis.com/problems/rockpaperscissors","interactive":false,"timeLimit":1000,"tests":[{"input":"2 4\n1 rock 2 paper\n1 scissors 2 paper\n1 rock 2 rock\n2 rock 1 scissors\n2 1\n1 rock 2 paper\n0\n","output":"0.333\n0.667\n\n0.000\n1.000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::IntoReal;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    if n == 0 {
        return;
    }
    let k = input.read_size();

    let mut wins = vec![0; n];
    let mut losses = vec![0; n];
    for _ in 0..k * n * (n - 1) / 2 {
        let p1 = input.read_size() - 1;
        let m1 = input.read_str();
        let p2 = input.read_size() - 1;
        let m2 = input.read_str();

        if m1 == m2 {
            continue;
        }
        let p1_wins = match (m1.as_slice(), m2.as_slice()) {
            (b"rock", b"scissors") => true,
            (b"scissors", b"paper") => true,
            (b"paper", b"rock") => true,
            _ => false,
        };
        if p1_wins {
            wins[p1] += 1;
            losses[p2] += 1;
        } else {
            wins[p2] += 1;
            losses[p1] += 1;
        }
    }
    if test_case != 1 {
        out.print_line(());
    }
    for i in 0..n {
        if wins[i] + losses[i] == 0 {
            out.print_line("-");
        } else {
            out.print_line(wins[i].into_real() / (wins[i] + losses[i]));
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);
    output.set_precision(3);

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
