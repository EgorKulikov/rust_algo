//{"name":"E - Exhibition Layout","group":"AtCoder - AtCoder Weekday Contest 0034 Beta","url":"https://atcoder.jp/contests/awc0034/tasks/awc0034_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 5 3\n2 4\n","output":"20\n"},{"input":"4\n10 10 20 30\n1 3 2\n","output":"110\n"},{"input":"8\n8 1 14 7 20 3 11 17\n5 2 9 1 4 8 3\n","output":"422\n"},{"input":"16\n42 7 81 19 63 5 97 28 54 12 76 33 88 21 69 40\n3 11 2 17 5 13 7 19 4 23 6 29 8 31 10\n","output":"11913\n"},{"input":"2\n1 10000000\n10000000\n","output":"99999990000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_long_vec(n);
    let w = input.read_long_vec(n - 1);

    let mut mem = Memoization2d::new(n, 1 << n, |mem, last, mask| -> i64 {
        if mask.count_ones() == 1 {
            0
        } else {
            let pos = mask.count_ones() as usize - 2;
            let mut res = 0;
            for i in 0..n {
                if i != last && mask.is_set(i) {
                    res.maxim(
                        (p[last] - p[i]).abs() * w[pos] + mem.call(i, mask.without_bit(last)),
                    );
                }
            }
            res
        }
    });
    let mut ans = 0;
    for i in 0..n {
        ans.maxim(mem.call(i, usize::all_bits(n)));
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
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
