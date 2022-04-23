//{"name":"A - Jogging","group":"AtCoder - Monoxer Programming Contest 2022（AtCoder Beginner Contest 249）","url":"https://atcoder.jp/contests/abc249/tasks/abc249_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3 3 6 2 5 10\n","output":"Takahashi\n"},{"input":"3 1 4 1 5 9 2\n","output":"Aoki\n"},{"input":"1 1 1 1 1 1 1\n","output":"Draw\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AJogging"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Ordering;

fn solve(input: &mut Input) {
    let a = input.read_int();
    let b = input.read_int();
    let c = input.read_int();
    let d = input.read_int();
    let e = input.read_int();
    let f = input.read_int();
    let x = input.read_int();

    let dist = |a: i32, b: i32, c: i32| -> i32 {
        let res = x / (b + c) * a * b;
        let rem = (x % (b + c)).min(b);
        res + rem * a
    };
    let takahashi = dist(b, a, c);
    let aoki = dist(e, d, f);
    out_line!(match takahashi.cmp(&aoki) {
        Ordering::Less => {
            "Aoki"
        }
        Ordering::Equal => {
            "Draw"
        }
        Ordering::Greater => {
            "Takahashi"
        }
    });
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
