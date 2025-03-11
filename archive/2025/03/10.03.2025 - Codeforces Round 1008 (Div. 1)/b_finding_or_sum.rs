//{"name":"B. Finding OR Sum","group":"Codeforces - Codeforces Round 1008 (Div. 1)","url":"https://mirror.codeforces.com/contest/2077/problem/B","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n\n3\n\n4\n\n1\n\n0\n\n1\n","output":"\n0\n\n1\n\n!\n\n4\n\n0\n\n!\n\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut ask = 0;
    for i in (0..30).step_by(2) {
        ask.set_bit(i);
    }
    out.print_line(ask);
    out.flush();
    let mut x = 0;
    let mut y = 0;
    let res = input.read_int();
    for j in (1..30).step_by(2) {
        match (res.is_set(j), res.is_set(j + 1)) {
            (true, false) => {}
            (false, true) => {
                x.set_bit(j);
            }
            (true, true) => {
                x.set_bit(j);
                y.set_bit(j);
            }
            (false, false) => {
                unreachable!()
            }
        }
    }
    out.print_line(ask * 2);
    out.flush();
    let res = input.read_int();
    for j in (2..30).step_by(2) {
        match (res.is_set(j), res.is_set(j + 1)) {
            (true, false) => {}
            (false, true) => {
                x.set_bit(j);
            }
            (true, true) => {
                x.set_bit(j);
                y.set_bit(j);
            }
            (false, false) => {
                unreachable!()
            }
        }
    }
    match (res.is_set(0), res.is_set(1)) {
        (false, false) => {}
        (true, false) => {
            x.set_bit(0);
        }
        (false, true) => {
            x.set_bit(0);
            y.set_bit(0);
        }
        (true, true) => {
            unreachable!()
        }
    }
    out.print_line("!");
    out.flush();
    let m = input.read_int();
    out.print_line((x | m) + (y | m));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
