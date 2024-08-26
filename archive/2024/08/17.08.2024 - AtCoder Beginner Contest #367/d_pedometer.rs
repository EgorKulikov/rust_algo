//{"name":"D - Pedometer","group":"AtCoder - AtCoder Beginner Contest 367","url":"https://atcoder.jp/contests/abc367/tasks/abc367_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n2 1 4 3\n","output":"4\n"},{"input":"2 1000000\n1 1\n","output":"0\n"},{"input":"9 5\n9 9 8 2 4 4 3 5 3\n","output":"11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPedometer"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);

    let mut qty = vec![0i64; m];
    let b = a.partial_sums();
    let mut ans = 0;
    for i in 0..n {
        ans += qty[b[i] % m];
        qty[b[i] % m] += 1;
    }
    for i in 0..n {
        qty[b[i] % m] -= 1;
        ans += qty[(b[i] + b[n]) % m];
    }
    out.print_line(ans);
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
