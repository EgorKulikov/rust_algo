//{"name":"Permutation Minimize Product","group":"Eolymp - Basecamp - Blitz Round #12","url":"https://eolymp.com/en/compete/qjgameovtl0l7dprd0o9efta48/problem/4","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n3 1 2\n5\n4 1 5 3 2\n6\n5 2 4 1 3 6\n4\n4 3 2 1\n","output":"2 1 1\n2 3 2 1 1\n2 2 2 2 2 3\n1 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let q = p.inv();
    let len = n.lower_sqrt();
    out.print_line_iter((0..n).map(|i| {
        let mut res = None;
        for j in i.saturating_sub(len)..=(i + len).min(n - 1) {
            if i != j {
                res.minim(p[i].abs_diff(p[j]) * i.abs_diff(j));
            }
        }
        for j in p[i].saturating_sub(len)..=(p[i] + len).min(n - 1) {
            if p[i] != j {
                res.minim(p[i].abs_diff(j) * i.abs_diff(q[j]));
            }
        }
        res
    }));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
