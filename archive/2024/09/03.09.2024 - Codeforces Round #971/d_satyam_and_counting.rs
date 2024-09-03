//{"name":"D. Satyam and Counting","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n1 0\n1 1\n3 0\n5 0\n2 1\n3\n0 0\n1 0\n3 0\n9\n1 0\n2 0\n3 0\n4 0\n5 0\n2 1\n7 1\n8 1\n9 1\n","output":"4\n0\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSatyamAndCounting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pts = input.read_size_pair_vec(n);

    let set = pts.iter().copied().collect::<HashSet<_>>();
    let mut ans = 0;
    for i in 0..=n {
        if set.contains(&(i, 0)) && set.contains(&(i, 1)) {
            ans += n - 2;
        }
    }
    for i in 1..n {
        if set.contains(&(i - 1, 0)) && set.contains(&(i, 1)) && set.contains(&(i + 1, 0)) {
            ans += 1;
        }
        if set.contains(&(i - 1, 1)) && set.contains(&(i, 0)) && set.contains(&(i + 1, 1)) {
            ans += 1;
        }
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
