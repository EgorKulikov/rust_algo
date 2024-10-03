//{"name":"Triangle Count (Easy)","group":"CodeChef - START154A","url":"https://www.codechef.com/START154A/problems/TRICOUNT1","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n5 2\n3\n5 2 4\n4\n5 2 4 12\n6\n3 1 6 5 50 17\n3\n100 2 69\n","output":"3\n7\n15\n53\n137\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TriangleCountEasy"}}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).sorted();

    let mut segments = Vec::new();
    for (a, b) in a.consecutive_iter() {
        segments.push((b - a + 1, a + b - 1));
    }
    segments.sort();
    let mut end = 0;
    let mut ans = 0;
    for (a, b) in segments {
        if b <= end {
            continue;
        }
        ans += b - end.max(a - 1);
        end = b;
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
