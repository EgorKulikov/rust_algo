//{"name":"A compressed array","group":"HackerEarth - March Circuits '22","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-22/algorithm/compressed-array-d6f321dd/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n6 7 3 6 6\n2\n4 4\n","output":"8\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACompressedArray"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    type Mod = ModInt7;
    let mut ans = Mod::zero();
    let mut last_ways = Mod::zero();
    let mut cur_ways = Mod::one();
    let mut same = Mod::one();
    let mut singles = Mod::one();
    for (&l, &c) in a.consecutive_iter() {
        if l != c {
            last_ways *= same;
            last_ways += cur_ways;
            cur_ways = Mod::one();
            same = Mod::one();
            singles += Mod::one();
        } else {
            same += Mod::one();
            cur_ways += same;
            singles = Mod::one();
        }
        ans -= singles;
        ans += last_ways * same + cur_ways;
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
