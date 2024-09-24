//{"name":"C. Yet Another Cool Pair Problem","group":"Codeforces - Theforces Round #34 (ABC-Forces)","url":"https://codeforces.com/gym/105350/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n2\n3\n4\n5\n6\n7\n8\n9\n10\n","output":"1\n1\n2\n2\n2\n2\n4\n4\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CYetAnotherCoolPairProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    for i in (0..30).rev() {
        if n.is_set(i) {
            let mut ans = 0;
            for j in (0..i).rev() {
                if ans.is_set(j + 1) {
                    continue;
                }
                let cand = ans.with_bit(j);
                if 2 * cand <= n {
                    ans = cand;
                }
            }
            out.print_line(ans);
            return;
        }
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
