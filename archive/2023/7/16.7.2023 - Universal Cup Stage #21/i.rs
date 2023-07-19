//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::memoization::Memoization;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::invertable::Invertable;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();
    let target = input.read_long();

    type Mod = ModIntF;
    let mut lim = [0; 11];
    for i in 2..=10 {
        lim[i] = target / i.into_i64();
    }
    let mut inv = [Mod::zero(); 20];
    for i in 1..20 {
        inv[i] = Mod::new(i.into_i32()).inv().unwrap();
    }
    let mut mem = Memoization::new(|f, n: i64| -> Mod {
        if n > target {
            return Mod::zero();
        }
        let mut nc = n;
        let mut sum = Mod::zero();
        let mut ways = 0;
        let mut zeroes = 0;
        while nc > 0 {
            let d = nc % 10;
            nc /= 10;
            ways += 1;
            if d == 0 {
                zeroes += 1;
            } else if lim[(d + 1).into_usize()] >= n {
                sum += f.call(n * (d + 1));
            }
        }
        (sum + Mod::new(ways)) * inv[(ways - zeroes).into_usize()]
    });
    out_line!(mem.call(n));
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
