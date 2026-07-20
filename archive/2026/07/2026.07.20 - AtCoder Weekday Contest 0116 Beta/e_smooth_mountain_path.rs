use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_str();
    let r = input.read_str();

    type Mod = ModInt7;
    let mut mem = Memoization3d::new(10, 10, 5002, |mem, digit, dif, rem| {
        if rem == 0 {
            Mod::one()
        } else {
            let mut res = Mod::zero();
            for d in 0..=9 {
                if digit.abs_diff(d) >= dif {
                    res += mem.call(d, digit.abs_diff(d), rem - 1);
                }
            }
            res
        }
    });
    let mut go = |n: Str, take: bool| -> Mod {
        let mut res = Mod::zero();
        for i in 0..n.len() - 1 {
            for j in 1..10 {
                res += mem.call(j, 0, i);
            }
        }
        for i in 1..(n[0] - b'0') as usize {
            res += mem.call(i, 0, n.len() - 1);
        }
        let mut diff = 0;
        let mut cur = (n[0] - b'0') as usize;
        for i in 1..n.len() {
            for j in 0..(n[i] - b'0') as usize {
                if cur.abs_diff(j) >= diff {
                    res += mem.call(j, j.abs_diff(cur), n.len() - i - 1);
                }
            }
            let next = (n[i] - b'0') as usize;
            if next.abs_diff(cur) < diff {
                return res;
            }
            diff = next.abs_diff(cur);
            cur = next;
        }
        if take {
            res += 1;
        }
        res
    };
    out.print_line(go(r, true) - go(l, false));
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
