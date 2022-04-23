//{"name":"E - RLE","group":"AtCoder - Monoxer Programming Contest 2022（AtCoder Beginner Contest 249）","url":"https://atcoder.jp/contests/abc249/tasks/abc249_e","interactive":false,"timeLimit":3000,"tests":[{"input":"3 998244353\n","output":"26\n"},{"input":"2 998244353\n","output":"0\n"},{"input":"5 998244353\n","output":"2626\n"},{"input":"3000 924844033\n","output":"607425699\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERLE"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{dynamic_value, out_line};

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let p = input.read_int();

    dynamic_value!(Module: i32);
    Module::set_val(p);
    type Mod = ModInt<i32, Module>;
    let mut ans = Arr2d::new(n + 1, n, None);
    let mut rec = RecursiveFunction2::new(|f, len: usize, comp: usize| -> (Mod, Mod) {
        match ans[(len, comp)] {
            Some(val) => val,
            None => {
                let cur = if len == 0 {
                    Mod::one()
                } else {
                    let mut res = Mod::zero();
                    if comp >= 2 {
                        res += f.call(len - 1, comp - 2).1;
                        if len >= 10 {
                            res -= f.call(len - 10, comp - 2).1;
                        }
                    }
                    if comp >= 3 && len >= 10 {
                        res += f.call(len - 10, comp - 3).1;
                        if len >= 100 {
                            res -= f.call(len - 100, comp - 3).1;
                        }
                    }
                    if comp >= 4 && len >= 100 {
                        res += f.call(len - 100, comp - 4).1;
                        if len >= 1000 {
                            res -= f.call(len - 1000, comp - 4).1;
                        }
                    }
                    if comp >= 5 && len >= 1000 {
                        res += f.call(len - 1000, comp - 5).1;
                    }
                    res * Mod::new(25)
                };
                let sum = if len == 0 {
                    cur
                } else {
                    cur + f.call(len - 1, comp).1
                };
                ans[(len, comp)] = Some((cur, sum));
                (cur, sum)
            }
        }
    });
    let ans = rec.call(n, n - 1).0 / Mod::new(25) * Mod::new(26);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
