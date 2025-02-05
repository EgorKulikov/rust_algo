//{"name":"Bachet's Game","group":"Kattis","url":"https://open.kattis.com/problems/bachetsgame","interactive":false,"timeLimit":1000,"tests":[{"input":"20 3 1 3 8\n21 3 1 3 8\n22 3 1 3 8\n23 3 1 3 8\n1000000 10 1 23 38 11 7 5 4 8 3 13\n999996 10 1 23 38 11 7 5 4 8 3 13\n","output":"Stan wins\nStan wins\nOllie wins\nStan wins\nStan wins\nOllie wins\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(m);

    let mut win = BitSet::new(n + 1);
    for i in 1..=n {
        for x in a.copy_iter() {
            if i >= x && !win[i - x] {
                win.set(i);
                break;
            }
        }
    }
    if win[n] {
        output!(out, "Stan wins");
    } else {
        output!(out, "Ollie wins");
    }
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
