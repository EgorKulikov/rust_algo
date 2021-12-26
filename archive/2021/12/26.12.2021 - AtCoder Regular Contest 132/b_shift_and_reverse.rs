//{"name":"B - Shift and Reverse","group":"AtCoder - AtCoder Regular Contest 132","url":"https://atcoder.jp/contests/arc132/tasks/arc132_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3 2\n","output":"2\n"},{"input":"2\n2 1\n","output":"1\n"},{"input":"10\n2 3 4 5 6 7 8 9 10 1\n","output":"3\n"},{"input":"12\n1 2 3 4 5 6 7 8 9 10 11 12\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BShiftAndReverse"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read();
    let p = input.read_vec::<usize>(n).dec_by_one();

    let pos_n = p.iter().find(&(n - 1)).unwrap();
    let pos_1 = p.iter().find(&0).unwrap();

    if pos_1 == 0 && pos_n == n - 1 {
        out_line!(0);
        return;
    }
    if pos_1 == n - 1 && pos_n == 0 {
        out_line!(1);
        return;
    }
    if pos_1 + 1 == pos_n {
        out_line!(1 + pos_n.min(n - pos_n));
        return;
    }
    out_line!(pos_1.min(n - pos_1 + 2));
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
