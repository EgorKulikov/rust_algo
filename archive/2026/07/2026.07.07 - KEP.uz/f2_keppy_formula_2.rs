use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable0, RecursiveFunction0};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let t = input.read_size();
    let k = s.copy_count(b'?');
    let lr = input.read_size_pair_vec(k);

    type Mod = ModInt7;
    let mut at = 0;
    let mut pos = 0;
    let mut tree = RecursiveFunction0::new(|rec| -> (Vec<Mod>, Mod) {
        let c = s[pos];
        pos += 1;
        match c {
            b'+' => {
                let (a, a_any) = rec.call();
                let (b, b_any) = rec.call();
                let mut res = vec![Mod::zero(); (a.len() + b.len() - 1).min(t + 1)];
                for i in a.indices() {
                    for j in b.indices() {
                        if i + j >= res.len() {
                            break;
                        }
                        res[i + j] += a[i] * b[j];
                    }
                }
                (res, a_any * b_any)
            }
            b'*' => {
                let (a, a_any) = rec.call();
                let (b, b_any) = rec.call();
                let mut res = vec![Mod::zero(); ((a.len() - 1) * (b.len() - 1)).min(t) + 1];
                for i in a.indices() {
                    for j in b.indices() {
                        if i * j >= res.len() {
                            break;
                        }
                        if i * j == 0 {
                            continue;
                        }
                        res[i * j] += a[i] * b[j];
                    }
                }
                res[0] = a[0] * b_any + b[0] * a_any - a[0] * b[0];
                (res, a_any * b_any)
            }
            b'0'..=b'9' => {
                let d = (c - b'0') as usize;
                if d > t {
                    (vec![Mod::zero()], Mod::one())
                } else {
                    let mut res = vec![Mod::zero(); d + 1];
                    res[d] = Mod::one();
                    (res, Mod::one())
                }
            }
            b'?' => {
                let (l, r) = lr[at];
                at += 1;
                let mut res = vec![Mod::zero(); t.min(r) + 1];
                for i in l..res.len() {
                    res[i] = Mod::one();
                }
                (res, Mod::from(r - l + 1))
            }
            _ => unreachable!(),
        }
    });
    out.print_line(tree.call().0.get(t).unwrap_or(&Mod::zero()));
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
