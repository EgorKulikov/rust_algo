//{"name":"A Rank Problem","group":"Kattis","url":"https://open.kattis.com/problems/rankproblem","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\nT4 T1\nT3 T1\nT5 T3\n","output":"T2 T4 T1 T5 T3\n"},{"input":"8 4\nT4 T1\nT1 T2\nT2 T3\nT3 T4\n","output":"T1 T2 T3 T4 T5 T6 T7 T8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut order = (1..=n).collect::<Vec<_>>();
    for _ in 0..m {
        scan!(input, "T@ T@", t1: usize, t2: usize);
        let p1 = order.copy_find(t1).unwrap();
        let p2 = order.copy_find(t2).unwrap();
        if p1 > p2 {
            order[p2..=p1].rotate_left(1);
        }
    }
    out.print_line_iter(order.iter_map(|x| format!("T{}", x)));
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
