//{"name":"D - Doubles","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2025#1 (AtCoder Beginner Contest 392)","url":"https://atcoder.jp/contests/abc392/tasks/abc392_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 1 2 3\n4 1 2 2 1\n6 1 2 3 4 5 6\n","output":"0.333333333333333\n"},{"input":"3\n5 1 1 1 1 1\n4 2 2 2 2\n3 1 1 2\n","output":"0.666666666666667\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::qty;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::rational::Rational;
use algo_lib::numbers::real::IntoReal;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let dice: Vec<Vec<usize>> = input.read_vec(n);

    let mut ans = Rational::new_int(0);
    for i in 0..n {
        let q = qty(&dice[i]);
        for j in 0..i {
            let mut cur = 0;
            for k in dice[j].copy_iter() {
                cur += q[k];
            }
            ans.maxim(Rational::new(
                cur as i64,
                (dice[i].len() * dice[j].len()) as i64,
            ));
        }
    }
    out.print_line(ans.into_real());
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
