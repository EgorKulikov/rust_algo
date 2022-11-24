//{"name":"C. Почти квадратные числа","group":"Yandex - Yandex Cup 2022: Алгоритм, спринт (квалификация)","url":"https://contest.yandex.ru/yacup/contest/42199/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"98\n","output":"2\n"},{"input":"10\n","output":"3\n"},{"input":"2022\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPochtiKvadratnieChisla"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();

    let mut ans = 3;
    let mut was = HashSet::new();
    for i in 1.. {
        let x = i * (i + 1);
        if x > n {
            break;
        }
        if x == n {
            ans = 1;
            break;
        }
        was.insert(x);
        if was.contains(&(n - x)) {
            ans = 2;
        }
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
