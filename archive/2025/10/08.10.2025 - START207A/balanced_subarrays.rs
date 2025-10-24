//{"name":"Balanced Subarrays","group":"CodeChef - START207A","url":"https://www.codechef.com/START207A/problems/BALSUBCON","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n2\n9\n","output":"3\n1 2 3\n5\n1 1 2 3 1\n10\n1 3 1 2 3 1 3 2 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut k = input.read_size();

    let mut ans = Vec::new();
    let mut len = 0;
    let mut next = 1;
    while k > 0 {
        let new_len = len + 1;
        let add = new_len / 3;
        if k >= add {
            k -= add;
            ans.push(next);
            len = new_len;
            next += 1;
            if next == 4 {
                next = 1;
            }
        } else {
            len = 1;
            if next == 1 {
                ans.push(2);
                ans.push(2);
                ans.push(2);
                next = 3;
            } else {
                ans.push(1);
                ans.push(1);
                ans.push(1);
                next = 2;
            }
        }
    }
    out.print_line(ans.len());
    out.print_line(&ans);
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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
