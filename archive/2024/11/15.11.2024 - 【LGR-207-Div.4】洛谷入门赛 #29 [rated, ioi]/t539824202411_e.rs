//{"name":"T539824 202411E 卡牌","group":"Luogu","url":"https://www.luogu.com.cn/problem/T539824?contestId=213071","interactive":false,"timeLimit":1000,"tests":[{"input":"8 4\n5 5 1 2 1\n1 2 2 4 5\n5 5 5 5 5\n1 2 3 4 5\n","output":"1 1 0 0 1\n0\n"},{"input":"15 5\n5 5 5 5 5\n4 4 4 4 4\n3 3 3 3 3\n2 2 2 2 2\n1 1 1 1 1\n","output":"1 1 1 1 1\n0\n"},{"input":"10 1\n1 2 3 3 3\n","output":"0 0 1 0 0\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T539824202411E"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut x = input.read_size();
    let n = input.read_size();

    let mut qty = [0; 5];
    for _ in 0..n {
        let mut max = 0;
        for v in input.iter().take(5) {
            if v <= x {
                max.maxim(v);
            }
        }
        if max > 0 {
            qty[max - 1] += 1;
            x -= max;
        }
    }
    out.print_line(qty);
    out.print_line(x);
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
