//{"name":"day2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day2"}}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        let s = input.read_line();
        data.push(s.parse_vec::<i32>());
    }

    fn check(v: &[i32]) -> bool {
        let mut incr = false;
        let mut decr = false;
        for (a, b) in v.consecutive_iter() {
            let delta = b - a;
            if delta.abs() > 3 || delta == 0 {
                return false;
            }
            if delta > 0 {
                incr = true;
            } else {
                decr = true;
            }
        }
        !incr || !decr
    }

    //part 1
    {
        let mut ans = 0;
        for v in &data {
            if check(v) {
                ans += 1;
            }
        }
        out.print_line(ans);
    }
    //part 2
    {
        let mut ans = 0;
        for v in &data {
            for i in v.indices() {
                let mut u = v.clone();
                u.remove(i);
                if check(&u) {
                    ans += 1;
                    break;
                }
            }
        }
        out.print_line(ans);
    }
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
