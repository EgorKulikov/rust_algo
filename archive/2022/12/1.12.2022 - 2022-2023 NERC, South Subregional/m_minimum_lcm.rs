//{"name":"M. Minimum LCM","group":"Codeforces - 2022-2023 ICPC, NERC, Южный четвертьфинал (онлайн-трансляция, правила ICPC, командное соревнование)","url":"https://codeforces.com/contest/1765/problem/M","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n9\n5\n10\n","output":"1 1\n3 6\n1 4\n5 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MMinimumLCM"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::primes::Factorize;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();

    let d = n.prime_divisors();
    out_line!(n / d[0].0, (d[0].0 - 1) * n / d[0].0);
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
