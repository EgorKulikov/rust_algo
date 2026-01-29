//{"name":"T724553 群星","group":"Luogu","url":"https://www.luogu.com.cn/problem/T724553?contestId=304744","interactive":false,"timeLimit":1500,"tests":[{"input":"3 3\n1 0 0\n","output":"2.00000\n1.33333\n2.66667\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization4;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::real::{IntoReal, Real};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(k);

    if k == 0 {
        return;
    }

    out.set_precision(5);
    if n <= 2 {
        out.print_line(Real(1.));
        if k == 2 {
            if (a[0] + a[1]) % 2 == 0 {
                out.print_line(Real(2.));
            } else {
                out.print_line(Real(1.));
            }
        }
        return;
    }
    let ways = (n * (n - 1) / 2).into_real();
    for i in 0..k {
        let mut mem = Memoization4::new(
            |mem, mut occ: u32, mut we: Option<usize>, step: usize, rem: i32| -> Real {
                if step == k {
                    return we.unwrap().into_real() + 1;
                }
                if rem == a[step] {
                    if step == i {
                        for j in 0..n {
                            if !occ.is_set(j) {
                                we = Some(j);
                                break;
                            }
                        }
                    } else if step < i {
                        for j in 0..n {
                            if !occ.is_set(j) {
                                occ.set_bit(j);
                                break;
                            }
                        }
                    }
                }
                if rem == 0 {
                    return mem.call(
                        occ,
                        we,
                        step + 1,
                        if step + 1 == k { 0 } else { a[step + 1] },
                    );
                }
                let mut res = Real(0.);
                for j in 0..n {
                    for k in 0..j {
                        let mut nocc = occ;
                        if occ.is_set(j) && !occ.is_set(k) {
                            nocc.unset_bit(j);
                            nocc.set_bit(k);
                        }
                        if !occ.is_set(j) && occ.is_set(k) {
                            nocc.set_bit(j);
                            nocc.unset_bit(k);
                        }
                        let nwe = if we == Some(j) {
                            Some(k)
                        } else if we == Some(k) {
                            Some(j)
                        } else {
                            we
                        };
                        res += mem.call(nocc, nwe, step, rem - 1) / ways;
                    }
                }
                res
            },
        );
        out.print_line(mem.call(0, None, 0, a[0]));
    }
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
