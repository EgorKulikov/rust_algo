//{"name":"D - Swap and Range Sum","group":"AtCoder - JPRS Programming Contest 2026#1 (AtCoder Beginner Contest 442)","url":"https://atcoder.jp/contests/abc442/tasks/abc442_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4\n2 7 1 8\n1 2\n2 1 2\n1 1\n2 2 4\n","output":"3\n17\n"},{"input":"8 10\n22 75 26 45 72 81 47 29\n2 2 7\n2 6 8\n2 4 4\n1 2\n2 1 3\n1 1\n2 2 4\n1 2\n1 4\n2 1 1\n","output":"346\n157\n45\n123\n142\n26\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    let mut ft = FenwickTree::from(a.as_slice());
    for _ in 0..q {
        let command = input.read_int();
        match command {
            1 => {
                let x = input.read_size() - 1;
                let left = ft.get(x..=x);
                let right = ft.get(x + 1..=x + 1);
                ft.add(x, right - left);
                ft.add(x + 1, left - right);
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                out.print_line(ft.get(l..r));
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
