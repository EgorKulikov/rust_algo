//{"name":"Tournament Rigging (Minimum)","group":"CodeChef - START211A","url":"https://www.codechef.com/START211A/problems/KUPSETMN","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 1\n2 2\n3 8\n4 11\n","output":"1 1 3 1 2 3 4\n2 2 3 1 2 3 4\n8 8 4 2 8 6 4 2 3 8 1 6 7 4 5\n11 5 11 5 9 1 11 5 7 15 9 3 1 13 11 5 6 8 7 16 15 10 9 3 4 1 2 13 14 12 11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let w = input.read_size();

    let mut ans = vec![0; (1 << (n + 1)) - 1];
    let mut at = 1;
    ans[Back(0)] = w;
    for i in (1..=(1 << n)).rev() {
        if i != w {
            ans[Back(at)] = i;
            at += 1;
        }
    }
    for i in (0..(1 << n) - 1).rev() {
        if ans[2 * i + 2] == w {
            ans[i] = w;
        } else {
            ans[i] = ans[2 * i + 1].min(ans[2 * i + 2]);
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
