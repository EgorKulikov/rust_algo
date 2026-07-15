use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_int_vec(n);

    fn go(a: &[usize], b: &[i32]) -> Vec<usize> {
        let mut cur_left = 0;
        let mut blue = Vec::new();
        let mut left = Vec::new();
        for i in a.indices() {
            if b[i] == 1 {
                blue.push(a[i]);
            } else {
                let mut at = blue.len();
                while at > cur_left && blue[at - 1] < a[i] {
                    at -= 1;
                }
                cur_left = at;
                left.push(at);
            }
        }
        left
    }
    let blues = b.copy_sum() as usize;
    let left = go(&a, &b);
    let right = go(&a.reversed(), &b.reversed()).reversed();
    type Mod = ModIntF;

    let mut mem = Memoization2d::new(left.len() + 1, blues + 1, |mem, p1, p2| -> Mod {
        if p1 == left.len() {
            Mod::one()
        } else {
            let mut res = Mod::zero();
            if p2 >= left[p1] {
                res += mem.call(p1 + 1, p2);
            }
            if p2 + right[p1] < blues {
                res += mem.call(p1, p2 + 1);
            }
            res
        }
    });
    out.print_line(mem.call(0, 0));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
