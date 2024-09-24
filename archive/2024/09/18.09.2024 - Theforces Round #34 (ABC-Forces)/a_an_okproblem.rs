//{"name":"A. An OK Problem","group":"Codeforces - Theforces Round #34 (ABC-Forces)","url":"https://codeforces.com/gym/105350/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4 6\n1 1\n5 7\n7 5\n11 22\n","output":"2\n0\n24\n0\n21464\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAnOKProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut ans = 0;
    for i in 0..n {
        if i + 4 > n {
            continue;
        }
        for j in 0..m {
            if j + 3 > m {
                continue;
            }
            for k in 0..n {
                if k + 4 > n {
                    continue;
                }
                for l in 0..m {
                    if l + 3 > m {
                        continue;
                    }
                    if i.abs_diff(k) >= 4 || j.abs_diff(l) >= 3 {
                        ans += 1;
                    }
                }
            }
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
