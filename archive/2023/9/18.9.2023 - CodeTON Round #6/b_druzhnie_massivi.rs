//{"name":"B. Дружные массивы","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2 3\n0 1\n1 2 3\n3 1\n1 1 2\n1\n","output":"0 1\n2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDruzhnieMassivi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_unsigned_vec(n);
    let b = input.read_unsigned_vec(m);

    let xor = a.iter().fold(0, |acc, &x| acc ^ x);
    let or = b.iter().fold(0, |acc, &x| acc | x);
    if n % 2 == 1 {
        out_line!(xor, xor | or);
    } else {
        out_line!(xor & (!or), xor);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

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
