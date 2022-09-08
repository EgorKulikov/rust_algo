//{"name":"E. Почти идеально","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2\n3\n50\n","output":"2\n4\n830690567\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPochtiIdealno"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    type Mod = ModIntF;
    let mut ones = Vec::with_capacity(n + 1);
    ones.push(Mod::one());
    ones.push(Mod::one());
    for i in 2..=n {
        let cur = ones[i - 1] + ones[i - 2] * Mod::from_index(i - 1);
        ones.push(cur);
    }
    let mut twos = Mod::one();
    let mut ans = Mod::zero();
    let c = Combinations::new(n + 1);
    for i in 0..=n / 4 {
        ans += twos * ones[n - 4 * i] * c.c(n - i * 2, 2 * i);
        twos *= Mod::new(2);
        twos *= Mod::from_index(2 * i + 1);
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
}
//END MAIN
