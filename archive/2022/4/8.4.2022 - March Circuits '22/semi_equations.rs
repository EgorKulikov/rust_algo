//{"name":"Semi-equations","group":"HackerEarth - March Circuits '22","url":"https://www.hackerearth.com/de/challenges/competitive/march-circuits-22/algorithm/semi-equation-bec91fcd/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1\n7\n","output":"1\n33\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SemiEquations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    type Mod = ModInt7;
    let matrix = Matrix::new(&[
        &[Mod::one(), Mod::one(), Mod::zero()],
        &[Mod::one(), Mod::zero(), Mod::zero()],
        &[Mod::one(), Mod::zero(), Mod::one()],
    ]);
    out_line!(
        matrix
            .power(n)
            .mult(&Matrix::column(&[Mod::one(), Mod::zero(), Mod::zero()]))[(2, 0)]
    );
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
