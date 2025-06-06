//{"name":"Can you reach","group":"CodeChef - START179A","url":"https://www.codechef.com/START179A/problems/FRMN","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n3 4 5 4 3\n4\n1 2 1 2\n","output":"10\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_int_vec(n);

    let mut ans = 0;
    let mut start = 0;
    let mut prev = 0;
    for i in 1..n - 1 {
        if h[i] < h[i - 1] && h[i] < h[i + 1] || h[i] > h[i - 1] && h[i] > h[i + 1] {
            let q = i - start + 1;
            ans += q * (q - 1) / 2;
            ans += (q - 1) * (start - prev);
            prev = start;
            start = i;
        }
    }
    let q = n - start;
    ans += q * (q - 1) / 2;
    ans += (q - 1) * (start - prev);
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
