//{"name":"D. Сделай круглым","group":"Codeforces - Codeforces Round  #834 (Div. 3)","url":"https://codeforces.com/contest/1759/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n6 11\n5 43\n13 5\n4 16\n10050 12345\n2 6\n4 30\n25 10\n2 81\n1 7\n","output":"60\n200\n65\n60\n120600000\n10\n100\n200\n100\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSdelaiKruglim"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::primes::PrimesTrait;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut n = input.read_long();
    let mut m = input.read_long();

    let mut two = n.max_power(2);
    let mut five = n.max_power(5);

    while two < five && m >= 2 {
        n *= 2;
        m /= 2;
        two += 1;
    }
    while five < two && m >= 5 {
        n *= 5;
        m /= 5;
        five += 1;
    }
    while m >= 10 {
        n *= 10;
        m /= 10;
    }
    out_line!(n * m);
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
