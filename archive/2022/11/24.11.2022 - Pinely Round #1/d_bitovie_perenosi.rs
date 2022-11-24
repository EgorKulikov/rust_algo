//{"name":"D. Битовые переносы","group":"Codeforces - Pinely Round 1 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1761/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"3 1\n","output":"15\n"},{"input":"3 0\n","output":"27\n"},{"input":"998 244\n","output":"573035660\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBitoviePerenosi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    type Mod = ModInt7;

    if k == 0 {
        out_line!(Mod::new(3).power(n));
        return;
    }
    if k == n {
        out_line!(Mod::new(3).power(n - 1));
        return;
    }
    let c: Combinations<Mod> = Combinations::new(n + 1);
    let mut ans = Mod::zero();
    for i in 1..=k {
        if n - k >= i {
            ans += c.c(n - k, i) * c.c(k - 1, i - 1) * Mod::new(3).power(n - 2 * i);
        }
        if n - k >= i - 1 {
            ans += c.c(n - k, i - 1) * c.c(k - 1, i - 1) * Mod::new(3).power(n + 1 - 2 * i);
        }
    }
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
