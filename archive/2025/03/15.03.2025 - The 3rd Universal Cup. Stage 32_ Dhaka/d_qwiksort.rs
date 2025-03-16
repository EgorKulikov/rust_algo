//{"name":"D. Qwiksort","group":"Universal Cup - The 3rd Universal Cup. Stage 32: Dhaka","url":"https://contest.ucup.ac/contest/1939/problem/10217","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n1 2 3 4 5 10 9 8 7 6\n2\n1 2 3 4\n","output":"7\n1 5\n3 7\n5 9\n6 10\n1 5\n3 7\n1 5\n6\n1 2\n2 3\n3 4\n1 2\n2 3\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _a = input.read_size_vec(2 * n);

    let mut ans = Vec::new();
    let k = n / 2 + 1;
    let l = k + n - 1;
    let kk = (n + 1) / 2 + 1;
    let ll = kk + n - 1;
    ans.push((n + 1, 2 * n));
    ans.push((k, l));
    ans.push((1, n));
    ans.push((kk, ll));
    ans.push((n + 1, 2 * n));
    ans.push((k, l));
    ans.push((1, n));
    ans.push((kk, ll));
    ans.push((n + 1, 2 * n));
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
