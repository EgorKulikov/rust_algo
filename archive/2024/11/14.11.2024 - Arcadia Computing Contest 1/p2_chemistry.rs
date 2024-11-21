//{"name":"P2 - Chemistry","group":"DMOJ - Arcadia Computing Contest 1","url":"https://dmoj.ca/problem/ahscc1p2","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n","output":"4\n1 2\n3 4\n"},{"input":"1 5\n","output":"2\n1 2 1 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P2Chemistry"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    if n == 1 || m == 1 {
        let ans = Arr2d::generate(n, m, |i, j| (i + j) % 2 + 1);
        out.print_line(if n == 1 && m == 1 { 1 } else { 2 });
        out.print_line(ans);
    } else {
        let ans = Arr2d::generate(n, m, |i, j| (2 * i + j) % 4 + 1);
        out.print_line(4);
        out.print_line(ans);
    }
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
