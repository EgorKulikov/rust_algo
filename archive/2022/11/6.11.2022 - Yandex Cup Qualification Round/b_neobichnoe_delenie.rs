//{"name":"B. Необычное деление","group":"Yandex - Yandex Cup 2022: Алгоритм, спринт (квалификация)","url":"https://contest.yandex.ru/yacup/contest/42199/problems/B/","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n10 1\n36 3\n11 2\n1000000000000000000 7\n","output":"10\n6\n4\n262143\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNeobichnoeDelenie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::number_iterator::iterate;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();
    let t = input.read_long();

    let mut ans = 0;
    'outer: for (mut pre, zero, _) in iterate(1, n) {
        while pre > 0 {
            let d = pre % 10;
            if d % t != 0 {
                continue 'outer;
            }
            pre /= 10;
        }
        ans += (9 / t + 1).power(zero);
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
