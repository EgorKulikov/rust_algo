//{"name":"P5 - Sheets","group":"DMOJ - JOI '05 Final Round","url":"https://dmoj.ca/problem/joi05fp5","interactive":false,"timeLimit":250,"tests":[{"input":"5 1\n0 0 3 2\n1 1 2 5\n0 4 6 5\n3 3 5 6\n5 0 7 6\n","output":"29\n"},{"input":"5 2\n0 0 3 2\n1 1 2 5\n0 4 6 5\n3 3 5 6\n5 0 7 6\n","output":"29\n38\n"},{"input":"2 2\n0 0 8 9\n0 0 9 8\n","output":"80\n36\n"},{"input":"3 2\n2 2 8 9\n3 0 4 9\n5 0 7 9\n","output":"48\n34\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::compress::{compress, Compressed};
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let r = input.read_int();
    let rects = input.read_vec::<(i64, i64, i64, i64)>(n);

    let (x1, y1, x2, y2) = rects.detuple();
    let Compressed {
        order: mut x,
        arrs: [mut x1, mut x2],
    } = compress([&x1, &x2]);
    let Compressed {
        order: mut y,
        arrs: [mut y1, mut y2],
    } = compress([&y1, &y2]);

    let mut area = 0;
    let mut perimeter = 0;
    for _ in 0..2 {
        let mut delta = vec![0; y.len()];
        let mut adds = vec![Vec::new(); x.len()];
        let mut removes = vec![Vec::new(); x.len()];
        for i in 0..n {
            adds[x1[i]].push((y1[i], y2[i]));
            removes[x2[i]].push((y1[i], y2[i]));
        }
        for i in 0..x.len() - 1 {
            for (y1, y2) in adds[i].drain(..) {
                delta[y1] += 1;
                delta[y2] -= 1;
            }
            for (y1, y2) in removes[i].drain(..) {
                delta[y1] -= 1;
                delta[y2] += 1;
            }
            let len = x[i + 1] - x[i];
            let mut cov = 0;
            for j in 0..y.len() - 1 {
                if cov == 0 && delta[j] != 0 {
                    perimeter += 2 * len;
                }
                cov += delta[j];
                if cov != 0 {
                    area += (y[j + 1] - y[j]) * len;
                }
            }
        }
        swap(&mut x1, &mut y1);
        swap(&mut x2, &mut y2);
        swap(&mut x, &mut y);
    }
    out.print_line(area / 2);
    if r == 2 {
        out.print_line(perimeter);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
