//{"name":"D. Дореми и игра с колышками","group":"Codeforces - Codeforces Global Round 24","url":"https://codeforces.com/contest/1764/problem/D","interactive":false,"timeLimit":1500,"tests":[{"input":"4 100000007\n","output":"16\n"},{"input":"1145 141919831\n","output":"105242108\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDoremiIIgraSKolishkami"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{dynamic_value, out_line};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let p = input.read_int();

    dynamic_value!(Module: i32 = p);
    type Mod = ModInt<i32, Module>;

    let mut ans = Mod::zero();
    let mut res = HashMap::new();
    let c: Combinations<Mod> = Combinations::new(n + 1);
    let mut f = |has_to_go: usize, can_go: usize| -> Mod {
        if let Some(&res) = res.get(&(has_to_go, can_go)) {
            return res;
        }
        let mut val = Mod::zero();
        for i in 0..=can_go {
            val += c.c(can_go, i) * c.fact(i + has_to_go);
        }
        res.insert((has_to_go, can_go), val);
        val
    };
    for i in 1..=n / 2 {
        for j in 1..=n / 2 {
            let k = n - i - j;
            if 2 * k >= n {
                continue;
            }
            let has_to_go = i - 1 + j - 1;
            let can_go = if k == 0 { 0 } else { k - 1 };
            ans += f(has_to_go, can_go);
        }
    }
    ans *= Mod::from_index(n);
    out_line!(ans);
}

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
}
//END MAIN
