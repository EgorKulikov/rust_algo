//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::prime_fft::PrimeFFT;
use algo_lib::out_line;
// use algo_lib::misc::memoization_2d::Memoization2d;
// use algo_lib::misc::recursive_function::Callable2;
// use algo_lib::numbers::num_traits::bit_ops::BitOps;
// use algo_lib::numbers::num_traits::primitive::Primitive;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n * m + 1);
    let mut fft = PrimeFFT::<Mod>::new();
    let mut base = Vec::with_capacity(m + 1);
    for i in 0..=m {
        base.push(c.c(m, i) * Mod::new(-1).power(m - i) * c.inv_fact(i));
    }
    let res = fft.power(&base, n);
    let mut ans = Mod::zero();
    for i in 0..=n * m {
        ans += res[i] * c.fact(i);
    }
    ans *= c.fact(m).power(n);
    out_line!(ans);
}

/*#[test]
fn test() {
    for n in 2..=5 {
        for m in 1..=(20 / n) {
            let mut mem = Memoization2d::new(n * m + 1, 1 << (n * m), |f, step, mask| {
                if step == n * m {
                    1i64
                } else {
                    let mut res = 0;
                    for i in 0..n * m {
                        if i / m != step / m && !mask.is_set(i) {
                            res += f.call(step + 1, mask.with_bit(i));
                        }
                    }
                    res
                }
            });
            let mut cur = mem.call(0, 0);
            for _ in 0..n {
                for i in 1..=m.into_i64() {
                    assert_eq!(cur % i, 0);
                    cur /= i;
                }
            }
            println!("{} {} {}", n, m, cur);
        }
    }
}*/

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
