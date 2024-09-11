//{"name":"Sequence Search","group":"CodeChef - START151A","url":"https://www.codechef.com/START151A/problems/SEQSEARCH","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n18 13 2\n50 30 7\n61 41 6\n","output":"13\n110\n123\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SequenceSearch"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let k = input.read_long();

    let mut left = 0;
    let mut right = i64::MAX / 2;
    while left < right {
        let mid = (left + right) / 2;
        let bs = mid / b + 1;
        let as_ = if mid >= a { (mid - a) / b + 1 } else { 0 };
        let total = bs + as_;
        if total >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out.print_line(left);
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
