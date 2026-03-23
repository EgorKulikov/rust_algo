//{"name":"A Game in the Park","group":"Kattis","url":"https://open.kattis.com/problems/agameinthepark","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 1 4\n5 -5 5\n9 0 12\n0 1 2\n","output":"30\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_int_table(4, n);

    let mut mem = Memoization3d::new(n + 1, 5, 16, |mem, col, row, mask| -> i32 {
        if col == n {
            0
        } else if row == 4 {
            mem.call(col + 1, 0, mask)
        } else {
            let mut ans = mem.call(col, row + 1, mask.without_bit(row));

            if !mask.is_set(row) && (row == 0 || !mask.is_set(row - 1)) {
                ans.maxim(x[(row, col)] + mem.call(col, row + 1, mask.with_bit(row)));
            }
            ans
        }
    });
    out.print_line(mem.call(0, 0, 0));
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
