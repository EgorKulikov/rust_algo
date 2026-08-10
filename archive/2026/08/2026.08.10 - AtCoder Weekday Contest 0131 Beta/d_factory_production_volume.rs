use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let m = input.read_unsigned();
    let p = input.read_unsigned_vec(n);

    dynamic_value!(Modulo: u32 = m);
    type Mod = ModInt<Modulo>;
    let mut left = vec![Mod::one()];
    for i in p.copy_iter() {
        let next = left[Back(0)] * i;
        left.push(next);
    }
    let mut right = vec![Mod::one()];
    for i in p.copy_rev() {
        let next = right[Back(0)] * i;
        right.push(next);
    }
    right.reverse();

    for _ in 0..q {
        let x = input.read_size() - 1;
        let v = input.read_unsigned();
        out.print_line(left[x] * right[x + 1] * v);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
