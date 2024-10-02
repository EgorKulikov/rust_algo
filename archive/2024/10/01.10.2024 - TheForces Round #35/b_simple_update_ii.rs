//{"name":"B. Simple Update - II","group":"Codeforces - TheForces Round #35 (LOL-Forces)","url":"https://codeforces.com/gym/105390/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n10\n3\n100\n4\n0110\n5\n10100\n7\n1010010\n","output":"0\n1\n3 0\n3 1\n5 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSimpleUpdateII"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let ones = s.iter().count_eq(&b'1');

    if ones == n {
        out.print_line(vec![0; n / 2]);
        return;
    }
    let prefix = s.iter().take_while(|&c| c == b'1').count();
    let suffix = s.iter().rev().take_while(|&c| c == b'1').count();
    let mut ans = Vec::with_capacity(n / 2);
    for k in 1..=n / 2 {
        if ones >= n - k {
            ans.push(0);
            continue;
        }
        ans.push((n - prefix - suffix).div_ceil(k) - 1);
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
