use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let s = input.read_str();
    let mut c = input.read_unsigned_vec(n);
    let abx = input.read_vec::<(Str, Str, Str)>(n);

    fn convert(s: &Str) -> u32 {
        let mut res = 0;
        for i in s.indices() {
            if s[i] == b'1' {
                res.set_bit(i);
            }
        }
        res
    }
    let ops = Vec::with_gen(n, |i| {
        (convert(&abx[i].0), convert(&abx[i].1), convert(&abx[i].2))
    });
    let start = convert(&s);
    type Mod = ModIntF;
    let mut qty = DefaultHashMap::new(Mod::zero());
    let mut rec = RecursiveFunction3::new(|rec, s: u32, done: u32, q: Vec<u32>| {
        if q.len() == n {
            qty[q] += 1;
            return;
        }
        for i in 0..n {
            if done.is_set(i) {
                continue;
            }
            let (a, b, x) = ops[i];
            if (s & a) != a || (s & b) != 0 {
                continue;
            }
            let mut q = q.clone();
            q.push((s ^ x).count_ones());
            rec.call(s ^ x, done.with_bit(i), q);
        }
    });
    rec.call(start, 0, Vec::new());

    for _ in 0..q {
        let t = input.read_size() - 1;
        let y = input.read_unsigned();
        c[t] = y;
        out.print_line(qty.get(&c));
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
