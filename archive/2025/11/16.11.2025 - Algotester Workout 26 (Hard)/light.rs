//{"name":"Light","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/147299","interactive":false,"timeLimit":1000,"tests":[{"input":"2 3\n1 2 1\n0 1 0\n","output":"1 0\n0 0\n0 0 0\n0 1 0\n"},{"input":"4 5\n2 2 2 2 2\n2 2 2 2 2\n2 2 2 2 2\n2 2 2 2 2\n","output":"1 1 1 1\n0 0 0 0\n1 1 1 1 1\n0 0 0 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_table(n, m);

    let min = a.copy_min();
    for r_min in 0..=min {
        let min = min - r_min;
        let mut row = (0..n)
            .map(|i| a.row(i).min().copied().unwrap() - r_min)
            .collect::<Vec<_>>();
        let mut col = (0..m)
            .map(|j| a.col(j).min().copied().unwrap() - min)
            .collect::<Vec<_>>();

        let r1 = Vec::with_gen(n, |i| {
            let cur = row[i].min(1);
            row[i] -= cur;
            cur
        });
        let c1 = Vec::with_gen(m, |j| {
            let cur = col[j].min(1);
            col[j] -= cur;
            cur
        });
        if row.copy_max() > 1 || col.copy_max() > 1 {
            continue;
        }
        out.print_line(r1);
        out.print_line(row);
        out.print_line(c1);
        out.print_line(col);
        return;
    }
    unreachable!();
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
