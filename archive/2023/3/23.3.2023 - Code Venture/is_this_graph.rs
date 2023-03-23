//{"name":"Is this Graph","group":"CodeChef - CDVN2023","url":"https://www.codechef.com/CDVN2023/problems/GRAPH_007","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2 4 1\n1 2 1\n","output":"1\n2\n"},{"input":"2\n2 4 0\n2 2 0\n","output":"0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IsThisGraph"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    type Mod = ModInt7;

    let mut arr = Arr2d::new(10, 10, Mod::zero());
    for i in 0..10 {
        let a = i + 1;
        let b = 2 * a;
        if b > 0 && b <= 10 {
            arr[(i, b - 1)] += Mod::one();
        }
        let b = 4 * a;
        if b > 0 && b <= 10 {
            arr[(i, b - 1)] += Mod::one();
        }
        let b = a / 2;
        if b > 0 && b <= 10 {
            arr[(i, b - 1)] += Mod::one();
        }
        let b = a / 4;
        if b > 0 && b <= 10 {
            arr[(i, b - 1)] += Mod::one();
        }
        let b = a + 1;
        if b > 0 && b <= 10 {
            arr[(i, b - 1)] += Mod::one();
        }
        let b = a - 1;
        if b > 0 && b <= 10 {
            arr[(i, b - 1)] += Mod::one();
        }
    }
    let mat = Matrix::from(arr);

    let q = input.read_size();
    for _ in 0..q {
        let a = input.read_size() - 1;
        let b = input.read_size() - 1;
        let p = input.read_size();
        out_line!(mat.power(p)[(a, b)]);
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
