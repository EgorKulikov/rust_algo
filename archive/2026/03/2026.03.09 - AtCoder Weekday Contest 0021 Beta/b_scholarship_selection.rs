//{"name":"B - Scholarship Selection","group":"AtCoder - AtCoder Weekday Contest 0021 Beta","url":"https://atcoder.jp/contests/awc0021/tasks/awc0021_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n100 80 100 90\n2 1 2\n3 2 3 4\n0\n","output":"1\n3\n0\n"},{"input":"5 6\n50 50 50 30 70 70\n3 1 2 3\n2 5 6\n4 1 4 5 6\n1 4\n0\n","output":"1\n5\n5\n4\n0\n"},{"input":"8 10\n1000000000 999999999 500000000 750000000 1000000000 250000000 750000000 100000000 999999999 500000000\n5 1 2 3 4 5\n3 6 7 8\n4 2 5 9 10\n0\n2 1 5\n6 1 2 3 4 5 6\n1 8\n10 1 2 3 4 5 6 7 8 9 10\n","output":"1\n7\n5\n0\n1\n1\n8\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_int_vec(m);

    for _ in 0..n {
        let k = input.read_size();
        let a = input.read_size_vec(k).dec().sorted();
        let mut max = 0;
        for a in a.copy_iter() {
            max.maxim(p[a]);
        }
        if max == 0 {
            out.print_line(0);
        } else {
            for i in 0..k {
                if p[a[i]] == max {
                    out.print_line(a[i] + 1);
                    break;
                }
            }
        }
    }
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
