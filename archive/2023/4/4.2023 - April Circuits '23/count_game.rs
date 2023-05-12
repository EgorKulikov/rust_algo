//{"name":"Count Game","group":"HackerEarth - April Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/april-circuits-23/algorithm/weighted-tree-function-d4f02544/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 2 2\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CountGame"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n);

    let mut base = Vec::new();
    for mut i in a {
        for &j in &base {
            i.minim(i ^ j);
        }
        if i > 0 {
            base.push(i);
        }
    }
    type Mod = ModInt7;
    out_line!(Mod::new(2).power(n - base.len()));
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
