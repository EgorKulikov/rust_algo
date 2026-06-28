use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::debug;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_4d::Memoization4d;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::real::{IntoReal, Real};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut matrix = Arr2d::new(52, 52, Real(0.));
    let mut c = Arr2d::new(53, 53, Real(0.));
    for i in 0..=52 {
        c[(i, 0)] = Real(1.);
        for j in 1..=i {
            c[(i, j)] = c[(i - 1, j - 1)] + c[(i - 1, j)];
        }
    }
    let mut mem = Memoization4d::new(
        52,
        53,
        53,
        52,
        |mem, pos: usize, left: usize, right: usize, target: usize| -> Real {
            if target == 0 {
                if pos == 0 {
                    left.into_real() / (left + right)
                } else {
                    Real(0.)
                }
            } else {
                let mut res = Real(0.);
                if right > 0 {
                    res += mem.call(pos, left, right - 1, target - 1) * right / (left + right);
                }
                if pos > 0 {
                    res += mem.call(pos - 1, left - 1, right, target - 1) * left / (left + right);
                }
                res
            }
        },
    );
    let pw = Real(2.).power(52);
    for i in 0..52 {
        for j in 0..=i {
            for k in 0..52 {
                matrix[(i, k)] += c[(52, j)] / pw * mem.call(i - j, 52 - j, j, k);
            }
        }
        for j in i + 1..=52 {
            for k in 0..52 {
                matrix[(i, k)] += c[(52, j)] / pw * mem.call(i, j, 52 - j, k);
            }
        }
    }

    let t = input.read_size();
    let mut deltas = Vec::new();
    for _ in 0..t {
        let mut delta = 0;
        // let mut uniform = Real(0.);
        for _ in 0..1000 {
            let s = input.read_str();
            for i in 0..52 {
                let c = s[i];
                let id = if c.is_ascii_uppercase() {
                    (c - b'A') as usize
                } else {
                    (c - b'a') as usize + 26
                };
                // if id == 0 || id == 51 {
                //     riffle += matrix[(id, i)];
                //     uniform += Real(1.) / 52;
                // }
                if id == 0 {
                    delta += i as i64;
                    // break;
                }
                if id == 51 {
                    delta -= i as i64;
                    // break;
                }
            }
        }
        // eprintln!("{}", (riffle - uniform).round());
        deltas.push(delta);
        if delta <= -2200 {
            out.print_line("riffle");
        } else {
            out.print_line("uniform");
        }
    }
    deltas.sort();
    debug!(deltas);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
