//{"name":"Ball Game","group":"CodeChef - START151A","url":"https://www.codechef.com/START151A/problems/BALL_GAME","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1 4\n1 2\n3\n1 7 5\n1 3 2\n4\n3 4 8 12\n1 2 4 5\n2\n1000000000 999999999\n999999999 999999998\n","output":"2\n2\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BallGame"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| a[i]);
    let mut stack = Vec::new();
    for i in order {
        let (a, b) = (a[i], b[i]);
        let time = Rational::new(a, b);
        while let Some(&prev) = stack.last() {
            if prev > time {
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(time);
    }
    out.print_line(stack.len());
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
