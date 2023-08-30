//{"name":"H. Obada's Problem","group":"Codeforces - Aleppo Collegiate Programming Contest 2023 V.2","url":"https://codeforces.com/gym/104544/problem/H","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n5\n7\n","output":"0\n326\n22212\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HObadasProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::inverses;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

type Mod = ModInt7;
type PreCalc = Vec<Mod>;

fn solve(input: &mut Input, _test_case: usize, data: &PreCalc) {
    let n = input.read_size();

    out_line!(data[n - 1]);
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut pre_calc = Vec::with_capacity(1000000);
    let mut fact = Mod::one();
    let mut mult = Mod::zero();
    let inv = inverses::<Mod>(1000001);
    for i in 1..=1000000 {
        mult += Mod::from_index(i - 1) * inv[i];
        fact *= Mod::from_index(i);
        pre_calc.push(mult * fact);
    }

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
