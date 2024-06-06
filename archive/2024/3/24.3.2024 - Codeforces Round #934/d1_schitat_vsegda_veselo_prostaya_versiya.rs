//{"name":"D1. Считать всегда весело (простая версия)","group":"Codeforces - Codeforces Round 934 (Div. 1)","url":"https://codeforces.com/contest/1943/problem/D1","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n3 1 998244853\n4 1 998244353\n3 2 998244353\n343 343 998244353\n","output":"4\n7\n10\n456615865\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1SchitatVsegdaVeseloProstayaVersiya"}}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let p = input.read_int();

    dynamic_value!(DynMod: i32 = p);
    type Mod = ModInt<i32, DynMod>;
    let mut mem = Memoization3d::new(
        n + 1,
        k + 1,
        k + 1,
        |mem, pos, last, cant_close| -> (Mod, Mod, Mod) {
            let base = if pos == n {
                if cant_close == 0 {
                    1.into()
                } else {
                    0.into()
                }
            } else {
                let flat = mem.call(pos + 1, last, 0).1
                    - if cant_close == 0 {
                        Mod::zero()
                    } else {
                        mem.call(pos + 1, cant_close - 1, 0).1
                    };
                let stair = mem.call(pos + 1, k, k - last).2 - mem.call(pos + 1, last, 0).2;
                flat + stair
            };
            let flat = if last > cant_close {
                base + mem.call(pos, last - 1, cant_close).1
            } else {
                base
            };
            let stair = if cant_close > 0 {
                base + mem.call(pos, last - 1, cant_close - 1).2
            } else {
                base
            };
            (base, flat, stair)
        },
    );
    out.print_line(mem.call(0, 0, 0).0);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
