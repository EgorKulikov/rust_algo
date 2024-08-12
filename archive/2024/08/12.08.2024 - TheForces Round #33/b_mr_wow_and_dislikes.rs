//{"name":"B. Mr. Wow and Dislikes","group":"Codeforces - TheForces Round #33(Wow-Forces)","url":"https://codeforces.com/gym/105293/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 2 1\n1 2 2\n4 3 1\n4 -7 9 -10\n5 8 3\n3 6 9 6 10\n6 11 6\n100 43 34 4 56 89\n6 12 6\n-1 -2 -3 -4 0 -11\n","output":"2\n4\n2\n12\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMrWowAndDislikes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long_vec(n);

    let mut left = 0;
    let mut right = 1_000_000_000;
    while left < right {
        let mid = (left + right) / 2;
        let mut required = 0;
        for &c in &c {
            if c > mid * b {
                required += (c - mid * b).upper_div(a - b);
            }
        }
        if required > mid {
            left = mid + 1;
        } else {
            right = mid;
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
