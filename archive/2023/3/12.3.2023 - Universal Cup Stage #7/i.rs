//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt9;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

type Mod = ModInt9;

fn solve(input: &mut Input, _test_case: usize, ans: &[Mod]) {
    let n = input.read_size();

    out_line!(Mod::one() - ans[n - 1]);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }

    let mut ways_same = vec![Mod::one()];
    let mut last = Mod::one();
    for i in 2..=100_000 {
        last *= Mod::new(2).power(i) - Mod::one();
        last /= Mod::from_index(i);
        last /= Mod::new(2).power(2 * i - 2);
        ways_same.push(last);
    }

    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1, &ways_same);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

#[test]
fn test() {
    let mut last = Rational::<i128>::new(1, 1);
    for i in 2..10 {
        let mut cur = 0i128;
        for j in 1..i {
            cur += (i - j) * 2i128.power(j.max(2) - 2);
        }
        println!("{} {}", i, cur);
        // last *= Rational::<i128>::new(cur, 1);
        // last /= Rational::new(2.pow(i - 1));
        // last /= Rational::new(i);
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
