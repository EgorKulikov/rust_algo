//{"name":"K. P to Q","group":"Universal Cup - The 3rd Universal Cup. Stage 33: India","url":"https://contest.ucup.ac/contest/1954/problem/10275","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n1 2\n2 1\n3\n1 2 3\n1 2 3\n3\n2 1 3\n3 1 2\n","output":"\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _p = input.read_size_vec(n);
    let q = input.read_size_vec(n);

    let mut ans = Vec::new();
    for i in (1..=n).rev() {
        ans.push((i, i));
    }
    for i in 1..=n {
        let mut pos = 1;
        for j in 0..n {
            if q[j] == i {
                break;
            }
            if q[j] < i {
                pos += 1;
            }
        }
        ans.push((i, pos));
    }
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
