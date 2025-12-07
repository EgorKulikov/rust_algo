//{"name":"day7","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_lines();

    let mut pos = vec![s[0].copy_position(|x| x == b'S').unwrap()];
    let mut ans = 0;
    for i in s.indices() {
        let mut next_pos = Vec::new();
        for j in pos {
            if s[i][j] == b'^' {
                ans += 1;
                next_pos.push(j - 1);
                next_pos.push(j + 1);
            } else {
                next_pos.push(j);
            }
        }
        next_pos.dedup();
        pos = next_pos;
    }
    out.print_line(ans);
    let mut qty = vec![0i64; s[0].len()];
    qty[s[0].copy_position(|x| x == b'S').unwrap()] = 1;
    for i in s.indices() {
        let mut next_qty = vec![0; s[i].len()];
        for j in s[i].indices() {
            if s[i][j] == b'^' {
                next_qty[j - 1] += qty[j];
                next_qty[j + 1] += qty[j];
            } else {
                next_qty[j] += qty[j];
            }
        }
        qty = next_qty;
    }
    out.print_line(qty.copy_sum());
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
