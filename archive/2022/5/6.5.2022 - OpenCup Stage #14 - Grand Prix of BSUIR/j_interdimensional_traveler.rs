//{"name":"J. Interdimensional Traveler","group":"Yandex - Grand Prix of BSUIR","url":"https://official.contest.yandex.com/opencupXXII/contest/37753/problems/J/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2 3\n4 6\n8 10\n5 6\n","output":"714250005\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JInterdimensionalTraveler"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);
    let p = input.read_int_pair_vec(n);

    type Mod = ModInt7;

    let mut ans = Mod::one();
    for ((s, t), a) in p.into_iter().zip(a) {
        if 2 * s <= t {
            continue;
        }
        let p = Mod::new(s) / Mod::new(t);
        let alpha = (Mod::one() - p) / p;
        ans *= alpha.power(a);
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
