//{"name":"C. Penchick and BBQ Buns","group":"Codeforces - Codeforces Round 987 (Div. 2)","url":"https://codeforces.com/contest/2031/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3\n12\n","output":"-1\n1 2 3 6 10 2 7 6 10 1 7 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPenchickAndBBQBuns"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    if n % 2 == 1 {
        if n <= 25 {
            out.print_line(-1);
        } else {
            let mut ans = vec![0; n];
            ans[0] = 1;
            ans[9] = 1;
            ans[25] = 1;
            ans[26] = 2;
            ans[22] = 2;
            ans[23] = 3;
            ans[24] = 3;
            let mut next = 4;
            for i in (1..9).step_by(2) {
                ans[i] = next;
                ans[i + 1] = next;
                next += 1;
            }
            for i in (10..22).step_by(2) {
                ans[i] = next;
                ans[i + 1] = next;
                next += 1;
            }
            for i in (27..n).step_by(2) {
                ans[i] = next;
                ans[i + 1] = next;
                next += 1;
            }
            out.print_line(ans);
        }
    } else {
        out.print_line_iter((0..n).map(|i| i / 2 + 1));
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
