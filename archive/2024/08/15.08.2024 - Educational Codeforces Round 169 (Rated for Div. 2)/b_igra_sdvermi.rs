//{"name":"B. Игра с дверьми","group":"Codeforces - Educational Codeforces Round 169 (Rated for Div. 2)","url":"https://codeforces.com/contest/2004/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2\n3 4\n2 5\n2 5\n3 7\n6 7\n4 5\n2 8\n","output":"1\n3\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BIgraSDvermi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l1 = input.read_size();
    let r1 = input.read_size();
    let l2 = input.read_size();
    let r2 = input.read_size();

    if l1.max(l2) > r1.min(r2) {
        out.print_line(1);
    } else {
        let mut ans = r1.min(r2) - l1.max(l2) + 2;
        if l1 == l2 {
            ans -= 1;
        }
        if r1 == r2 {
            ans -= 1;
        }
        out.print_line(ans);
    }
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
