//{"name":"E. Rectangular Congruence","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"http://codeforces.com/contest/1734/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n0 0\n","output":"0 1\n0 0\n"},{"input":"3\n1 1 1\n","output":"1 2 2\n1 1 0\n1 0 1\n"},{"input":"5\n1 4 1 2 4\n","output":"1 0 1 3 4\n1 4 3 1 0\n2 4 1 0 2\n1 2 2 2 2\n2 2 0 1 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERectangularCongruence"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{dynamic_value, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    dynamic_value!(Module: usize = n);
    type Mod = ModInt<usize, Module>;
    let b = input.read_vec::<Mod>(n);

    let mut ans = Arr2d::new(n, n, Mod::zero());
    for i in 0..n {
        let mut cur = b[i];
        for j in 0..n {
            ans[(i, (i + j) % n)] = cur;
            cur += Mod::new(i);
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
