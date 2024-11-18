//{"name":"T539822 202411C K/D/A","group":"Luogu","url":"https://www.luogu.com.cn/problem/T539822?contestId=213071","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5/3/8\n3/4/7\n3/3/13\n","output":"1\n"},{"input":"2\n10/1/9\n10/0/1\n","output":"2\n"},{"input":"2\n9/6/3\n8/5/2\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T539822202411CKDA"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = None;
    for i in 1..=n {
        scan!(input, "@/@/@", k: i64, d: i64, a: i64);
        let strength = if k - d >= 10 {
            k * (k - d) + a
        } else if k >= d {
            (k - d + 1) * 3 + a
        } else {
            a * 2
        };
        ans.maxim((strength, i));
    }
    out.print_line(ans.unwrap().1);
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
