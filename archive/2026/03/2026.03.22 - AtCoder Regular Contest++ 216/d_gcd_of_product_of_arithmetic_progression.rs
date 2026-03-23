//{"name":"D - GCD of Product of Arithmetic Progression","group":"AtCoder - AtCoder Regular Contest++ 216","url":"https://atcoder.jp/contests/arc216/tasks/arc216_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 1 1 1\n4 2 2 6\n2026 3 22 216\n","output":"6\n128\n114347907\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::mod_int::{ModInt, ModIntF};
use algo_lib::numbers::num_utils::{factorials, UpperDiv};
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let di = divisor_table(1_000_001);
    type Mod = ModIntF;
    let f = factorials::<Mod>(1_000_001);

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let mut b = input.read_size();
        let mut c = input.read_size();
        let mut d = input.read_size();

        let g = gcd(b, gcd(c, d));
        b /= g;
        c /= g;
        d /= g;
        let mut ans = Mod::from(g).power(n) * f[n];
        let mut x = b;
        let od = d;
        while x > 1 {
            let p = di[x];
            let mut q = 1;
            while x % p == 0 {
                x /= p;
                q *= p;
            }
            let mut nn = n;
            while nn >= p {
                nn /= p;
                ans /= Mod::from(p).power(nn);
            }
            if d % p == 0 {
                while d % p == 0 {
                    d /= p;
                }
                continue;
            }
            // while q < n {
            //     q *= p;
            // }
            let mut first = true;
            while q > 1 {
                dynamic_value!(Modulo: u32 = q as u32);
                type Mod2 = ModInt<Modulo>;
                let i = (-Mod2::from(c) / od).val();
                if (i as usize) < n {
                    let mut times = (n - i as usize).upper_div(q);
                    ans *= Mod::from(p).power(times);
                    if first {
                        while times >= p {
                            times /= p;
                            ans *= Mod::from(p).power(times);
                        }
                        first = false;
                    }
                }
                q /= p;
            }
        }
        let mut gg = d;
        while gg != 1 {
            let p = di[gg];
            while gg % p == 0 {
                gg /= p;
            }
            let mut nn = n;
            while nn >= p {
                nn /= p;
                ans /= Mod::from(p).power(nn);
            }
        }
        out.print_line(ans);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
