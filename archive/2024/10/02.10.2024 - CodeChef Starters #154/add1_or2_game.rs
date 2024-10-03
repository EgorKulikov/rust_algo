//{"name":"Add 1 or 2 Game","group":"CodeChef - START154A","url":"https://www.codechef.com/START154A/problems/ADD12GAME","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1\n3\n","output":"ALICE\nBOB\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Add1Or2Game"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut rec = RecursiveFunction2::new(|rec, x: usize, is_alice: bool| -> bool {
        if x == n {
            false
        } else if x > n {
            true
        } else if is_alice {
            rec.call(x + 1, false) || rec.call(x + 2, false)
        } else {
            rec.call(x + 1, true) && rec.call(x + 2, true)
        }
    });
    if rec.call(0, true) {
        out.print_line("ALICE");
    } else {
        out.print_line("BOB");
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
