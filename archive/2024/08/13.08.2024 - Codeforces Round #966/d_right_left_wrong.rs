//{"name":"D. Right Left Wrong","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\n3 5 1 4 3 2\nLRLLLR\n2\n2 8\nLR\n2\n3 9\nRL\n5\n1 2 3 4 5\nLRLRR\n","output":"18\n10\n0\n22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRightLeftWrong"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let s = input.read_str();

    let b = a.partial_sums();
    let mut l = 0;
    let mut r = n;
    let mut ans = 0;
    while l < r {
        while l < n && s[l] == b'R' {
            l += 1;
        }
        while r > 0 && s[r - 1] == b'L' {
            r -= 1;
        }
        if l < r {
            ans += b[r] - b[l];
            l += 1;
            r -= 1;
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
