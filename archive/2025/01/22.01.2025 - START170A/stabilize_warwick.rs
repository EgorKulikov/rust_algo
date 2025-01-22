//{"name":"Stabilize Warwick","group":"CodeChef - START170A","url":"https://www.codechef.com/START170A/problems/STABWAR","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n5 1 2 6\n3\n1 1 1\n3\n4 1 4\n","output":"4\n4 2\n3 1\n4 2\n2 4\n0\n3\n3 3\n3 3\n3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _p = input.read_size_vec(n);

    let mut ans = Vec::new();
    ans.push((1, 2));
    for _ in 0..n - 2 {
        ans.push((1, 1));
    }
    ans.push((3, 2));
    ans.push((2, 1));
    for i in (4..=n).step_by(2) {
        ans.push((i, 2));
    }
    for i in (5..=n).step_by(2) {
        ans.push((i, 3));
    }
    ans.push((1, 3));
    out.print_line(ans.len());
    out.print_per_line(&ans);
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
//END MAIN
