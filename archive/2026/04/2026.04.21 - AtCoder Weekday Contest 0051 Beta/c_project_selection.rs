//{"name":"C - Project Selection","group":"AtCoder - AtCoder Weekday Contest 0051 Beta","url":"https://atcoder.jp/contests/awc0051/tasks/awc0051_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1 100\n50 80\n60 90\n40 50\n1 2\n","output":"140\n"},{"input":"5 3 200\n30 100\n50 150\n40 120\n60 200\n80 180\n1 2\n2 3\n4 5\n","output":"420\n"},{"input":"8 6 500\n100 500\n80 400\n120 600\n90 450\n70 350\n110 550\n60 300\n150 800\n1 2\n1 3\n2 4\n3 4\n5 6\n7 8\n","output":"2350\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let k = input.read_long();
    let cp = input.read_long_pair_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    let mut ans = 0;
    'outer: for i in usize::iter_all(n) {
        let mut cost = 0;
        for j in 0..n {
            if i.is_set(j) {
                cost += cp[j].0;
            }
        }
        if cost > k {
            continue;
        }
        for (a, b) in edges.copy_iter() {
            if i.is_set(a) && i.is_set(b) {
                continue 'outer;
            }
        }
        let mut profit = 0;
        for j in 0..n {
            if i.is_set(j) {
                profit += cp[j].1;
            }
        }
        ans.maxim(profit);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
