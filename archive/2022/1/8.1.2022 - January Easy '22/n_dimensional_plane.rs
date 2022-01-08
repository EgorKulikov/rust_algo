//{"name":"N-dimensional plane","group":"HackerEarth - January Easy '22","url":"https://www.hackerearth.com/challenges/competitive/january-easy-22/algorithm/counting-in-n-dimensional-b13b1ee2/","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n2\n1 1\n3\n2 3 1\n2\n1 2\n","output":"2\n60\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NDimensionalPlane"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

type Mod = ModInt7;

fn solve(input: &mut Input, _test_case: usize, c: &Combinations<Mod>) {
    let n = input.read();
    let dest = input.read_vec::<isize>(n);

    let mut at = 0;
    let mut ans = Mod::one();
    for i in dest {
        let i = i.abs().into_usize();
        at += i;
        ans *= c.c(at, i);
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let c = Combinations::new(200001);
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1, &c);
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
