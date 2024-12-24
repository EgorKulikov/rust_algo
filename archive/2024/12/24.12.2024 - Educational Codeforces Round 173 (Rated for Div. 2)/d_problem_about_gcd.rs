//{"name":"D. Problem about GCD","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 8 2\n4 8 3\n4 8 4\n5 7 6\n","output":"4 6\n-1 -1\n4 8\n6 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DProblemAboutGCD"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_long();
    let r = input.read_long();
    let g = input.read_long();

    let ll = l.upper_div(g) * g;
    let rr = r / g * g;
    if ll > rr {
        out.print_line((-1, -1));
        return;
    }
    let spread = (rr - ll) / g;
    for delta in (0..=spread).rev() {
        for i in 0..=spread - delta {
            if gcd(ll + i * g, ll + (i + delta) * g) == g {
                out.print_line((ll + i * g, ll + (i + delta) * g));
                return;
            }
        }
    }
    out.print_line((-1, -1));
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
