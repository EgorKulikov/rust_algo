//{"name":"T724556 Silent End","group":"Luogu","url":"https://www.luogu.com.cn/problem/T724556?contestId=304744","interactive":false,"timeLimit":4000,"tests":[{"input":"5 7\n1 2 4\n1 3 5\n5 4 4\n2 5 7\n5 1 1\n4 2 2\n1 5 7\n","output":"YES\n2 3 4 5 6\n"},{"input":"4 6\n1 2 1\n3 2 2\n4 2 3\n1 3 5\n1 4 6\n4 1 4\n","output":"NO\n"},{"input":"3 4\n1 2 1\n3 2 4\n3 1 2\n2 3 2\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let uvw = input.read_vec::<(usize, usize, i32)>(m).dec();

    let mut x = vec![0; n];
    for (_, v, w) in uvw.copy_iter() {
        x[v] = w;
    }
    let mut y = vec![0; n];
    for (u, v, w) in uvw.copy_iter() {
        if x[u] >= x[v] {
            y[v] = w + 1;
        } else {
            y[v] = w - 1;
        }
    }
    for (u, v, w) in uvw.copy_iter() {
        if y[u] >= y[v] {
            if w + 1 != y[v] {
                out.print_line(false);
                return;
            }
        } else {
            if w - 1 != y[v] {
                out.print_line(false);
                return;
            }
        }
    }
    out.print_line(true);
    out.print_line(y);
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
