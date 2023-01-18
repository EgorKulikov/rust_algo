//{"name":"D. Много точных квадратов","group":"Codeforces - VK Cup 2022 - Отборочный раунд (Engine)","url":"https://codeforces.com/contest/1781/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n5\n1 2 3 4 5\n5\n1 6 13 22 97\n1\n100\n5\n2 5 10 17 26\n","output":"2\n5\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMnogoTochnikhKvadratov"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut cand = HashSet::new();
    for &i in &a {
        for &j in &a {
            if i == j {
                break;
            }
            let d = i - j;
            let mut x = 1;
            while x * x <= d {
                if d % x == 0 {
                    let y = d / x;
                    if x % 2 == y % 2 {
                        let alpha = (x + y) / 2;
                        if alpha * alpha >= i {
                            cand.insert(alpha * alpha - i);
                        }
                    }
                }
                x += 1;
            }
        }
    }

    let mut ans = 1;
    for i in cand {
        let mut cur = 0;
        for &j in &a {
            let d = i + j;
            let mut ds = (d as f64).sqrt() as i64;
            while (ds + 1) * (ds + 1) <= d {
                ds += 1;
            }
            if ds * ds == d {
                cur += 1;
                continue;
            }
            while ds * ds > d {
                ds -= 1;
            }
            if ds * ds == d {
                cur += 1;
            }
        }
        ans.maxim(cur);
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
