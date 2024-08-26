//{"name":"A - Median of Good Sequences","group":"AtCoder - Marubeni Programming Contest 2024 (AtCoder Regular Contest 183)","url":"https://atcoder.jp/contests/arc183/tasks/arc183_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n","output":"1 2 2 1\n"},{"input":"1 5\n","output":"1 1 1 1 1\n"},{"input":"6 1\n","output":"3 6 5 4 2 1\n"},{"input":"3 3\n","output":"2 2 2 1 3 3 3 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMedianOfGoodSequences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut ans = Vec::with_capacity(n * k);

    let mut qty = vec![k; n + 1];
    qty[0] = 0;
    if n % 2 == 0 {
        if n != 1 {
            ans.push(n / 2);
            qty[n / 2] -= 1;
        }
    } else {
        for _ in 0..k {
            ans.push((n + 1) / 2);
        }
        qty[(n + 1) / 2] = 0;
        if n != 1 {
            ans.push(n / 2);
            qty[n / 2] -= 1;
        }
    }
    for i in (1..=n).rev() {
        for _ in 0..qty[i] {
            ans.push(i);
        }
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
