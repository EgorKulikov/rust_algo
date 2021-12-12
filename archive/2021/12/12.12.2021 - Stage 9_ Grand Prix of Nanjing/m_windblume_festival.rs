//{"name":"M. Windblume Festival","group":"Yandex - Stage 9: Grand Prix of Nanjing","url":"https://official.contest.yandex.ru/opencupXXII/contest/33444/problems/M/","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n4\n1 -3 2 -4\n11\n91 66 73 71 32 83 72 79 84 33 93\n12\n91 66 73 71 32 83 72 79 84 33 33 93\n13\n91 66 73 71 32 83 72 79 84 33 33 33 93\n1\n0\n","output":"10\n713\n746\n779\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MWindblumeFestival"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<i64> = input.read_vec(n);

    if n == 1 {
        out_line!(a[0]);
        return;
    }
    let mut has_negative = false;
    let mut has_positive = false;
    let mut smallest = i64::MAX;
    let mut answer = 0i64;
    for i in a {
        if i <= 0 {
            has_negative = true;
        }
        if i >= 0 {
            has_positive = true;
        }
        smallest.minim(i.abs());
        answer += i.abs();
    }
    if !has_negative || !has_positive {
        answer -= 2 * smallest;
    }
    out_line!(answer);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
