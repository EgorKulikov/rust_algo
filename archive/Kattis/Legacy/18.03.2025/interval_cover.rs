//{"name":"Interval Cover","group":"Kattis","url":"https://open.kattis.com/problems/intervalcover","interactive":false,"timeLimit":2000,"tests":[{"input":"-0.5 1\n3\n-0.9 -0.1\n-0.2 2\n-0.7 1\n0 1\n3\n0 0.25\n0.25 0.75\n0.75 0.999\n0 1\n3\n0 0.25\n0.25 0.75\n0.75 1\n1 1\n1\n1 1\n","output":"1\n2\nimpossible\n3\n0 1 2\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_real();
    let b = input.read_real();
    let n = input.read_size();
    let ab = input
        .read_vec::<(Real, Real)>(n)
        .iter_enumerate()
        .collect::<Vec<_>>()
        .sorted_by_key(|(_, p)| *p);

    if a == b {
        for (id, (x, y)) in ab {
            if x <= a && a <= y {
                out.print_line(1);
                out.print_line(id);
                return;
            }
        }
        out.print_line("impossible");
        return;
    }
    let mut last = None;
    let mut from = a;
    let mut to = a;
    let mut ans = Vec::new();
    for (id, (x, y)) in ab {
        if x > from {
            if x > to {
                out.print_line("impossible");
                return;
            }
            ans.push(last.unwrap());
            last = None;
            from = to;
        }
        if to.maxim(y) {
            last = Some(id);
        }
        if to >= b {
            break;
        }
    }
    if to < b {
        out.print_line("impossible");
        return;
    }
    ans.push(last.unwrap());
    out.print_line(ans.len());
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
