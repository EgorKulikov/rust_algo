//{"name":"M. Rejection Sampling","group":"Universal Cup - The 3rd Universal Cup. Stage 22: Zhengzhou","url":"https://contest.ucup.ac/contest/1873/problem/9780","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2\n5 5 5\n","output":"0.666666666667\n0.666666666667\n0.666666666667\n"},{"input":"2 1\n1 4\n","output":"0.333333333333\n0.666666666667\n"},{"input":"4 2\n1 2 3 4\n","output":"0.310035697652\n0.473324044845\n0.574114878920\n0.642525378583\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MRejectionSampling"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_real();
    let a = input.read_real_vec(n);

    let mut left = Real(0.);
    let mut right = Real(1.);
    for _ in 0..100 {
        let p0: Real = (left + right) / 2;
        let mut sum_p = p0;
        for i in 1..n {
            sum_p += a[i] / (a[i] - a[0] + a[0] / p0);
        }
        if sum_p.0 > k.0 {
            right = p0;
        } else {
            left = p0;
        }
    }
    let p0 = (left + right) / 2;
    for i in 0..n {
        out.print_line(a[i] / (a[i] - a[0] + a[0] / p0));
    }
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
