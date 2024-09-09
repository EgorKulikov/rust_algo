//{"name":"P2 - Cheating","group":"DMOJ - Back to School '24","url":"https://dmoj.ca/problem/bts24p2","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 2 3\n1 2 2\n","output":"6 5\n"},{"input":"5 6\n1 2 2 7 8\n3 1 4 1 5\n","output":"11 11 8 10 10 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P2Cheating"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_size_vec(n);

    let mut delta = vec![0; m];
    let mut at = 0;
    for (a, b) in a.into_iter().zip(b) {
        delta[at] += a;
        at += b;
        if at >= m {
            delta[0] += a;
            at -= m;
        }
        delta[at] -= a;
    }
    let mut ans = Vec::with_capacity(m);
    let mut cur = 0;
    for i in 0..m {
        cur += delta[i];
        ans.push(cur);
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
