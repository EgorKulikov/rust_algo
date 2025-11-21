//{"name":"G. Sakura Adachi and Optimal Sequences","group":"Codeforces - Codeforces Round 1065 (Div. 3)","url":"https://codeforces.com/contest/2171/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"8\n6\n1 3 6 4 3 2\n3 7 10 4 4 8\n2\n1 1\n4 3\n5\n2 3 2 5 1\n18 13 10 30 7\n5\n5 4 3 6 2\n100 125 231 113 107\n4\n2 2 2 2\n2 2 2 2\n4\n1 1 1 1\n2 2 2 2\n7\n1 1 1 1 1 1 200000\n200000 200000 200000 200000 200000 200000 200000\n3\n542264 174876 441510\n641112 325241 995342\n","output":"17 827116\n3 1\n12 288\n35 567812\n0 1\n1 1\n1199994 0\n803045 366998\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::inverse_factorials;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_utils::factorials;
use algo_lib::value;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    value!(Modulo: u32 = 1_000_003);
    type Mod = ModInt<Modulo>;
    let f = factorials::<Mod>(1_000_003);
    let inv_f = inverse_factorials::<u32, Mod>(1_000_003);
    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let a = input.read_size_vec(n);
        let b = input.read_size_vec(n);

        let mut ans = Mod::one();
        let mut num_doubles = 100;
        for i in 0..n {
            let d = b[i] / a[i];
            num_doubles.minim(d.highest_bit());
        }
        let mut base = 0;
        let mut fact = vec![0; num_doubles];
        let mut ops = num_doubles;
        for i in 0..n {
            let mut b0 = b[i];
            for j in 0..num_doubles {
                if b0 % 2 == 1 {
                    fact[j] += 1;
                    ops += 1;
                }
                b0 /= 2;
            }
            ans *= inv_f[b0 - a[i]];
            base += b0 - a[i];
            ops += b0 - a[i];
        }
        if base >= f.len() {
            ans = Mod::zero();
        } else {
            ans *= f[base];
            for i in fact {
                ans *= f[i];
            }
        }
        out.print_line((ops, ans));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

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
    }
    eprint!("\x1B[0m");
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive | TaskType::RunTwice => true,
    }
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
