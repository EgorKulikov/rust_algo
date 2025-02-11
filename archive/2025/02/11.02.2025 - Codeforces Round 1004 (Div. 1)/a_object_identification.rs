//{"name":"A. Object Identification","group":"Codeforces - Codeforces Round 1004 (Div. 1)","url":"https://codeforces.com/contest/2066/problem/A","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n3\n2 2 3\n\n1\n\n0\n\n5\n5 1 4 2 3\n\n4\n\n4\n","output":"? 2 3\n\n? 1 2\n\n! A\n\n? 1 5\n\n? 5 1\n\n! B\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_size_vec(n);

    let xs = x.copy_iter().collect::<FxHashSet<_>>();
    for i in 1..=n {
        if !xs.contains(&i) {
            out.print_line(('?', i, i % n + 1));
            out.flush();
            if input.read_int() == 0 {
                out.print_line("! A");
            } else {
                out.print_line("! B");
            }
            out.flush();
            return;
        }
    }
    let p1 = x.copy_find(1).unwrap() + 1;
    let pn = x.copy_find(n).unwrap() + 1;
    out.print_line(('?', p1, pn));
    out.flush();
    let d = input.read_size();
    if d < n - 1 {
        out.print_line("! A");
        out.flush();
        return;
    }
    if d > n - 1 {
        out.print_line("! B");
        out.flush();
        return;
    }
    out.print_line(('?', pn, p1));
    out.flush();
    if input.read_size() == n - 1 {
        out.print_line("! B");
    } else {
        out.print_line("! A");
    }
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
