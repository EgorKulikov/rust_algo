//{"name":"Watering Grass","group":"Kattis","url":"https://open.kattis.com/problems/grass","interactive":false,"timeLimit":2000,"tests":[{"input":"8 20 2\n5 3\n4 1\n1 2\n7 2\n10 2\n13 3\n16 2\n19 4\n3 10 1\n3 5\n9 3\n6 1\n3 10 1\n5 3\n1 1\n9 1\n","output":"6\n2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_real();
    let w = input.read_real() / 2;
    let mut segs = Vec::new();
    for _ in 0..n {
        let x = input.read_real();
        let r = input.read_real();
        if r <= w {
            continue;
        }
        let delta_sq: Real = r * r - w * w;
        let delta = delta_sq.sqrt();
        segs.push((x - delta, x + delta));
    }

    segs.sort();
    let mut ans = 1;
    let mut cur = Real(0.);
    let mut next = cur;
    for (a, b) in segs {
        if a > cur {
            if a > next {
                out.print_line(-1);
                return;
            }
            ans += 1;
            cur = next;
        }
        next.maxim(b);
        if next >= l {
            break;
        }
    }
    if next < l {
        out.print_line(-1);
        return;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    Real::set_epsilon(1e-15);

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
