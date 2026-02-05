//{"name":"Grassland Fence","group":"Eolymp - Basecamp - Educational Round #7","url":"https://eolymp.com/en/compete/0fdopgb6et7g3allmrhbicn9cg/problem/9","interactive":false,"timeLimit":1000,"tests":[{"input":"7 3 4 5 6 7 8 9\n4 1 2 4 8\n4 7 4 4 4\n","output":"36.7544\n0.0000\n6.9282\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_real_vec(n);

    fn area(a: Real, b: Real, c: Real) -> Real {
        let s = (a + b + c) / Real(2.);
        if a > s || b > s || c > s {
            return Real(0.);
        }
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
    let mut mem = Memoization1d::new(1 << n, |mem, mask| {
        let mut res = Real(0.);
        for i in 0..n {
            if !mask.is_set(i) {
                continue;
            }
            for j in 0..i {
                if !mask.is_set(j) {
                    continue;
                }
                for k in 0..j {
                    if !mask.is_set(k) {
                        continue;
                    }
                    let a = l[i];
                    let b = l[j];
                    let c = l[k];
                    res.maxim(
                        area(a, b, c) + mem.call(mask.without_bit(i).without_bit(j).without_bit(k)),
                    );
                }
            }
        }
        res
    });
    out.print_line(mem.call(usize::all_bits(n)));
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    output.set_precision(4);
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
            while !input.is_empty() {
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
