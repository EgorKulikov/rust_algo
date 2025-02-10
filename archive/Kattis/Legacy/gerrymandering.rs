//{"name":"Gerrymandering","group":"Kattis","url":"https://open.kattis.com/problems/gerrymandering","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 100 200\n2 100 99\n3 100 50\n3 100 50\n2 100 98\n","output":"B 100 49\nA 1 197\nA 49 100\n0.1965897693\n"},{"input":"4 4\n3 100 99\n2 100 99\n1 100 99\n4 100 99\n","output":"A 0 99\nA 0 99\nA 0 99\nA 0 99\n0.4974874372\n"},{"input":"4 4\n4 99 100\n1 100 99\n3 100 99\n2 99 100\n","output":"A 0 99\nB 99 0\nA 0 99\nB 99 0\n0.0000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::IntoReal;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let p = input.read_size();
    let d = input.read_size();
    let votes: Vec<(usize, usize, usize)> = input.read_vec(p);

    let mut a = vec![0; d];
    let mut b = vec![0; d];
    for (d, ai, bi) in votes {
        a[d - 1] += ai;
        b[d - 1] += bi;
    }
    let mut a_wasted = 0;
    let mut b_wasted = 0;
    let mut total = 0;
    for i in 0..d {
        let threshold = (a[i] + b[i]) / 2 + 1;
        if a[i] > b[i] {
            a_wasted += a[i] - threshold;
            b_wasted += b[i];
            out.print_line(('A', a[i] - threshold, b[i]));
        } else {
            a_wasted += a[i];
            b_wasted += b[i] - threshold;
            out.print_line(('B', a[i], b[i] - threshold));
        }
        total += a[i] + b[i];
    }
    out.print_line(a_wasted.abs_diff(b_wasted).into_real() / total);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
