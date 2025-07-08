//{"name":"B - LCM","group":"AtCoder - AtCoder Regular Contest 200 (Div. 2)","url":"https://atcoder.jp/contests/arc200/tasks/arc200_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 3 5\n1 1 7\n8 6 11\n","output":"Yes\n2025 200\nNo\nYes\n20250615 200200\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::lcm;
use algo_lib::numbers::number_ext::{num_digs, Power};

type PreCalc = Arr3d<Option<(i128, i128)>>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let a1 = input.read_size();
    let a2 = input.read_size();
    let a3 = input.read_size();

    match data[(a1, a2, a3)] {
        Some((a, b)) => {
            out.print_line(true);
            out.print_line((a, b));
        }
        None => {
            out.print_line(false);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Arr3d::new(18, 18, 18, None);

    for i in 0..60 {
        for j in 0..38 {
            let a = 3i128.power(j) * 2i128.power(i);
            let d1 = num_digs(a);
            if d1 >= 18 {
                continue;
            }
            for k in 0..60 {
                for l in 0..38 {
                    let b = 3i128.power(l) * 2i128.power(k);
                    let d2 = num_digs(b);
                    if d2 >= 18 {
                        continue;
                    }
                    let l = lcm(a, b);
                    let d3 = num_digs(l);
                    if d3 < 18 {
                        pre_calc[(d1, d2, d3)] = Some((a, b));
                    }
                }
            }
        }
    }

    output.set_bool_output(BoolOutput::YesNo);

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
