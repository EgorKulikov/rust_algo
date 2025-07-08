//{"name":"Add 1 or 2","group":"CodeChef - START191A","url":"https://www.codechef.com/START191A/problems/SUB12","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n2\n3 5\n1 2\n3\n1 2 1\n2 1 1\n3\n27 72 99\n38 54 66\n2\n1000000000 1000000000\n1000000000 1000000000\n4\n2 2 5 4\n3 3 1 3\n2\n10 20\n1 1\n","output":"6\n3\n131\n2000000000\n7\n20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::first_true;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let ans = first_true(0, i32::MAX as i64, |ans| {
        let mut balance = 0;
        for i in 0..n {
            if a[i] > ans {
                return false;
            }
            let delta = ans - a[i];
            if delta >= b[i] {
                balance += (delta - b[i]) / 2;
            } else {
                balance -= b[i] - delta;
            }
        }
        balance >= 0
    });
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
