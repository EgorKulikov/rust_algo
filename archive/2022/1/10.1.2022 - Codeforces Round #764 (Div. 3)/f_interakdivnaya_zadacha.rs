//{"name":"F. Интеракдивная задача","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/F","interactive":true,"timeLimit":1000,"tests":[{"input":"3\n\n1\n","output":"+ 1\n\n! 3\n"},{"input":"5\n\n0\n\n0\n\n1\n","output":"+ 1\n\n+ 1\n\n+ 1\n\n! 5\n"},{"input":"10\n\n0\n\n0\n\n1\n\n2\n","output":"+ 2\n\n+ 2\n\n+ 3\n\n+ 8\n\n! 20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FInterakdivnayaZadacha"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    let mut added = 0;
    let mut from = 1;
    let mut to = n - 1;
    while from != to {
        let mid = (from + to + 1) / 2;
        let delta = n - (mid + added) % n;
        out_line!('+', delta);
        if input.read_usize() == (mid + added) / n {
            to = mid - 1;
        } else {
            from = mid;
        }
        added += delta;
    }
    out_line!('!', from + added);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
