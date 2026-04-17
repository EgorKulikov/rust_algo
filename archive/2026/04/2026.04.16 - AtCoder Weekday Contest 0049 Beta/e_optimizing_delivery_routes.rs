//{"name":"E - Optimizing Delivery Routes","group":"AtCoder - AtCoder Weekday Contest 0049 Beta","url":"https://atcoder.jp/contests/awc0049/tasks/awc0049_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n10 20 40\n","output":"30\n"},{"input":"4\n5 3 8 1\n","output":"13\n"},{"input":"8\n100 250 120 400 50 300 180 90\n","output":"950\n"},{"input":"15\n1000000000 0 500000000 250000000 750000000 125000000 875000000 62500000 937500000 31250000 968750000 15625000 984375000 7812500 992187500\n","output":"1921875000\n"},{"input":"1\n0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let v = input.read_size_vec(n);

    let mut mem = Memoization2d::new(n, 1 << n, |mem, pos, mask| {
        if mask == usize::all_bits(n) {
            0
        } else {
            let mut res = usize::MAX;
            for i in 0..n {
                if !mask.is_set(i) {
                    res.minim(
                        mem.call(i, mask.with_bit(i)) + v[pos].abs_diff(v[i]) * i.abs_diff(pos),
                    );
                }
            }
            res
        }
    });
    out.print_line(mem.call(0, 1));
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
