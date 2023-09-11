//{"name":"E1. salyg1n и массив (простая версия)","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/E1","interactive":true,"timeLimit":1000,"tests":[{"input":"2\n4 2\n\n6\n\n4\n\n6 6\n\n4\n","output":"? 1\n\n? 3\n\n! 2\n\n? 1\n\n! 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1Salyg1nIMassivProstayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut at = 1;
    let mut ans = 0;
    while at + 2 * k <= n {
        out_line!("?", at);
        ans ^= input.read_int();
        at += k;
    }
    let step = (n - at + 1 - k) / 2;
    for _ in 0..3 {
        out_line!("?", at);
        ans ^= input.read_int();
        at += step;
    }
    out_line!("!", ans);
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
    // input.skip_whitespace();
    // !input.peek().is_some()
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
