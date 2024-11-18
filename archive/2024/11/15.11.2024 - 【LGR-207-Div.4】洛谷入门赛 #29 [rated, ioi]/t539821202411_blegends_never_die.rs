//{"name":"T539821 202411B Legends Never Die","group":"Luogu","url":"https://www.luogu.com.cn/problem/T539821?contestId=213071","interactive":false,"timeLimit":1000,"tests":[{"input":"1 3 4 5\n","output":"No\nYes\nYes\n"},{"input":"2 3 1 0\n","output":"Yes\nYes\nYes\n"},{"input":"7 0 0 5\n","output":"Yes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T539821202411BLegendsNeverDie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let k = input.read_long();

    out.set_bool_output(BoolOutput::YesNo);
    out.print_line(a + b >= k);
    out.print_line(a + c >= k);
    out.print_line(b + c >= k);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
