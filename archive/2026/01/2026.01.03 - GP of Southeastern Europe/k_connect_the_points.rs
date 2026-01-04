//{"name":"K. Connect the Points","group":"Universal Cup - GP of Southeastern Europe","url":"https://contest.ucup.ac/contest/2828/problem/16125","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n0 0 3 0\n1 1 2 2\n0 2 1 2\n","output":"YES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pts = input.read_size_pair_vec(2 * n);

    let id = |x: usize, y: usize| -> Option<usize> {
        if x == 0 {
            Some(y)
        } else if y == n {
            Some(n + x)
        } else if x == n {
            Some(2 * n + n - y)
        } else if y == 0 {
            Some(3 * n + n - x)
        } else {
            None
        }
    };
    for i in 0..n {
        for j in i + 1..n {
            let (x0, y0) = pts[2 * i];
            let (x1, y1) = pts[2 * i + 1];
            let (x2, y2) = pts[2 * j];
            let (x3, y3) = pts[2 * j + 1];
            let i1 = id(x0, y0);
            let i2 = id(x1, y1);
            let i3 = id(x2, y2);
            let i4 = id(x3, y3);
            if let (Some(i1), Some(i2), Some(i3), Some(i4)) = (i1, i2, i3, i4) {
                let (i1, i2) = (i1.min(i2), i1.max(i2));
                let (i3, i4) = (i3.min(i4), i3.max(i4));
                if i1 < i3 && i3 < i2 && i2 < i4 {
                    out.print_line(false);
                    return;
                }
                if i3 < i1 && i1 < i4 && i4 < i2 {
                    out.print_line(false);
                    return;
                }
            }
        }
    }
    out.print_line(true);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
