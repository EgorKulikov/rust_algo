//{"name":"L. Neo-Nim","group":"Universal Cup - GP of Southeastern Europe","url":"https://contest.ucup.ac/contest/2828/problem/16126","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 2\n2 5 8\n4 3\n2 2 4 5\n1 5\n2\n","output":"Bob\nBob\nAna\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).sorted();

    if k == 2 {
        let mut q = vec![0; 3];
        for i in a.copy_iter() {
            q[i % 3] += 1;
        }
        if q[0] == n - 1 && q[2] == 1 {
            out.print_line("Ana");
        } else {
            out.print_line("Bob");
        }
        return;
    }

    if a.copy_count(1) > 0 {
        out.print_line("Bob");
        return;
    }
    if a.copy_count(2) > 1 {
        out.print_line("Bob");
        return;
    }
    if k == 3 {
        if a[0] >= 4 && a[n - 1] <= 5 && a.copy_count(5) % 2 == 0 {
            out.print_line("Bob");
            return;
        }
        if n > 1 && (a[0] == 2 && a[1] >= 4 && a[n - 1] <= 5 && a.copy_count(5) % 2 == 1) {
            out.print_line("Bob");
            return;
        }
        if a[0] == 2 && n > 1 {
            if a[n - 1] <= 6 && a[n - 2] <= 5 && a.copy_count(5) % 2 == 1 {
                out.print_line("Bob");
                return;
            }
        }
    }
    out.print_line("Ana");
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
