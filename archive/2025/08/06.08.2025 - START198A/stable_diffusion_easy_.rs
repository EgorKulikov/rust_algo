//{"name":"Stable Diffusion (Easy)","group":"CodeChef - START198A","url":"https://www.codechef.com/START198A/problems/STDIFEZ","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n5\n2 1 2 1 2\n6\n1 2 2 1 2 1\n","output":"3\n13\n20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = n * (n + 1) / 2;
    for i in 0..n {
        for j in (i + 4..=n).step_by(2) {
            let mut good = true;
            let len = j - i;
            for k in 0..len / 2 {
                if a[i + k] == a[j - 1 - k] {
                    good = false;
                    break;
                }
            }
            if a[i] == a[i + 1] || a[i + 1] == a[i + 2] {
                good = false;
            }
            if a[i + len / 2 - 1] == a[i + len / 2 - 2] {
                good = false;
            }
            for k in 0..len / 2 - 2 {
                if a[i + k] == a[i + k + 1]
                    && (a[i + k] == a[i + k + 2] || a[i + k + 2] == a[i + k + 3])
                {
                    good = false;
                    break;
                }
            }
            if good {
                ans -= 1;
            }
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
