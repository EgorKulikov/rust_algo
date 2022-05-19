//{"name":"Boundless Atek","group":"CS Academy - FIICode 2022 Round #1 – Powered by Atek Software","url":"https://csacademy.com/contest/fiicode-2022-round-1/task/fiicode-2022-b1/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n21 13\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BoundlessAtek"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_long_vec(n);

    a.sort_unstable();
    let base = a[0];
    let mut g = 0;
    for i in a.into_iter().skip(1) {
        g = gcd(i - base, g);
    }
    if g == 0 {
        g = -1;
    }
    out_line!(g);
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
