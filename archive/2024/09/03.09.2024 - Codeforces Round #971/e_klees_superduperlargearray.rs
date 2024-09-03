//{"name":"E. Klee's SUPER DUPER LARGE Array!!!","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 2\n7 2\n5 3\n1000000000 1000000000\n","output":"1\n5\n1\n347369930\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKleesSUPERDUPERLARGEArray"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let k = input.read_long();

    let f = |x: i64| -> i64 {
        ((k + x) * (k + x + 1) - k * (k - 1) / 2 - (k + n) * (k + n - 1) / 2).abs()
    };
    let mut left = 0;
    let mut right = n - 1;
    while right - left > 10 {
        let mid_left = (2 * left + right) / 3;
        let mid_right = (left + 2 * right) / 3;

        if f(mid_left) < f(mid_right) {
            right = mid_right;
        } else {
            left = mid_left;
        }
    }
    let mut ans = None;
    for i in left..=right {
        ans.minim(f(i));
    }
    out.print_line(ans);
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
