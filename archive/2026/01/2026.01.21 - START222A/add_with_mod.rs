//{"name":"Add with Mod","group":"CodeChef - START222A","url":"https://www.codechef.com/START222A/problems/ADDMOD","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n1 998244353\n2 998244353\n3 998244353\n4 998244353\n5 998244353\n6 383838383\n","output":"0\n12\n1431\n233024\n53497250\n247288531\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_5d::Memoization5d;
use algo_lib::misc::recursive_function::Callable5;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_unsigned();

    dynamic_value!(Modulo: u32 = p);
    type Mod = ModInt<Modulo>;
    let c = Combinations::<Mod>::new(n + 1);
    let mut mem = Memoization5d::new(
        n + 1,
        2,
        n + 1,
        n + 1,
        n + 1,
        |mem, pos, tp, a, b_left, b_right| -> (Mod, Mod) {
            if pos == n {
                if a == n && b_left + b_right == n {
                    (Mod::zero(), Mod::one())
                } else {
                    (Mod::zero(), Mod::zero())
                }
            } else if tp == 0 {
                let mut sum = Mod::zero();
                let mut qty = Mod::zero();
                for i in 0..=n - a {
                    let (call_sum, call_qty) = mem.call(pos, 1, a + i, b_left, b_right);
                    sum += call_sum * c.c(a + i, a);
                    qty += call_qty * c.c(a + i, a);
                }
                (sum, qty)
            } else {
                let mut sum = Mod::zero();
                let mut qty = Mod::zero();
                for i in 0..=n - b_left - b_right {
                    let add_left = (a - b_left).min(i);
                    let add_right = i - add_left;
                    let (call_sum, call_qty) =
                        mem.call(pos + 1, 0, a, b_left + add_left, b_right + add_right);
                    let add_pay = a - (b_left + add_left) + add_right * pos;
                    sum += (call_sum + call_qty * add_pay) * c.c(b_left + b_right + i, i);
                    qty += call_qty * c.c(b_left + b_right + i, i);
                }
                (sum, qty)
            }
        },
    );
    let (ans, qty) = mem.call(0, 0, 0, 0, 0);
    assert_eq!(qty, Mod::from(n).power(2 * n));
    out.print_line(ans);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
