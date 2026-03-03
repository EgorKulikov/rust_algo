//{"name":"D - Team Building","group":"AtCoder - AtCoder Weekday Contest 0017 Beta","url":"https://atcoder.jp/contests/awc0017/tasks/awc0017_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2 2\n10 8 6 5\n1 2 3\n3 4 2\n","output":"16\n"},{"input":"5 4 3\n100 90 80 70 60\n1 2 50\n1 3 40\n2 3 30\n4 5 10\n","output":"220\n"},{"input":"8 6 4\n1000000000 900000000 800000000 700000000 600000000 500000000 400000000 300000000\n1 2 500000000\n1 3 400000000\n2 3 300000000\n1 4 200000000\n5 6 100000000\n7 8 50000000\n","output":"2700000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);
    let uvb = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut ans = None;
    for mask in usize::iter_all(n) {
        if mask.count_ones() != k as u32 {
            continue;
        }
        let mut cur = 0;
        for i in 0..n {
            if mask.is_set(i) {
                cur += a[i];
            }
        }
        for (u, v, b) in uvb.copy_iter() {
            if mask.is_set(u) && mask.is_set(v) {
                cur -= b;
            }
        }
        ans.maxim(cur);
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
