use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let a = input.read_int_vec(n);

    type Mod = ModIntF;
    let mut cur = 0;
    let mut qty_same = 0;
    let mut qty_compl = 0;
    let c = Combinations::<Mod>::new(n + 1);
    let mut ans = Mod::one();
    let mut val_same = DefaultHashMap::new(0);

    for i in a {
        let rem = i % k;
        if cur == rem {
            qty_same += 1;
            val_same[i] += 1;
        } else if (cur + rem) % k == 0 {
            qty_compl += 1;
        } else {
            if (cur * 2) % k == 0 {
                for &j in val_same.values() {
                    ans *= c.c(qty_same, j);
                    qty_same -= j;
                }
            } else {
                ans *= c.c(qty_same + qty_compl, qty_same);
            }
            qty_same = 1;
            qty_compl = 0;
            cur = rem;
            val_same.clear();
            val_same[i] += 1;
        }
    }
    if (cur * 2) % k == 0 {
        for &j in val_same.values() {
            ans *= c.c(qty_same, j);
            qty_same -= j;
        }
    } else {
        ans *= c.c(qty_same + qty_compl, qty_same);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
