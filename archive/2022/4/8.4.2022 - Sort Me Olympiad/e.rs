//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

type Mod = ModInt7;

fn solve_impl(n: usize, k: usize) -> Mod {
    let c: Combinations<Mod> = Combinations::new((n + 1).max(11));

    let mut ans = Mod::zero();
    let mut prod = Mod::one();
    for i in 1..=10 {
        if k * i > n {
            break;
        }
        prod *= c.c(n - (i - 1) * k, k);
        let cur = prod * Mod::from_index(10 - i).power(n - k * i) * c.c(10, i);
        if i % 2 == 0 {
            ans -= cur;
        } else {
            ans += cur;
        }
    }
    ans
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    out_line!(solve_impl(n, k));
}

#[test]
fn test() {
    for n in 1..=100 {
        for k in 1..=n {
            solve_impl(n, k);
        }
    }
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
