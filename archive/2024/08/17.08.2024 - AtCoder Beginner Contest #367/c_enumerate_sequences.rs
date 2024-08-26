//{"name":"C - Enumerate Sequences","group":"AtCoder - AtCoder Beginner Contest 367","url":"https://atcoder.jp/contests/abc367/tasks/abc367_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n2 1 3\n","output":"1 1 2\n2 1 1\n2 1 3\n"},{"input":"1 2\n1\n","output":"\n"},{"input":"5 5\n2 3 2 3 2\n","output":"1 1 1 1 1\n1 2 2 3 2\n1 3 1 3 2\n1 3 2 2 2\n1 3 2 3 1\n2 1 2 3 2\n2 2 1 3 2\n2 2 2 2 2\n2 2 2 3 1\n2 3 1 2 2\n2 3 1 3 1\n2 3 2 1 2\n2 3 2 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CEnumerateSequences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let r = input.read_size_vec(n);

    let mut cur = Vec::with_capacity(n);
    let mut rec = RecursiveFunction::new(|rec, step| {
        if step == n {
            if cur.iter().sum::<usize>() % k == 0 {
                out.print_line(&cur);
            }
            return;
        }
        for i in 1..=r[step] {
            cur.push(i);
            rec.call(step + 1);
            cur.pop();
        }
    });
    rec.call(0);
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
