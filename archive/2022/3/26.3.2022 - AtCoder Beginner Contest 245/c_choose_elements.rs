//{"name":"C - Choose Elements","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n9 8 3 7 2\n1 6 2 9 5\n","output":"Yes\n"},{"input":"4 90\n1 1 1 100\n1 2 3 100\n","output":"No\n"},{"input":"4 1000000000\n1 1 1000000000 1000000000\n1 1000000000 1 1000000000\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CChooseElements"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_int();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let mut can_a = true;
    let mut can_b = true;
    for ((&pa, &a), (&pb, &b)) in a.consecutive_iter().zip(b.consecutive_iter()) {
        let mut next_a = false;
        let mut next_b = false;
        if can_a {
            if (a - pa).abs() <= k {
                next_a = true;
            }
            if (b - pa).abs() <= k {
                next_b = true;
            }
        }
        if can_b {
            if (a - pb).abs() <= k {
                next_a = true;
            }
            if (b - pb).abs() <= k {
                next_b = true;
            }
        }
        can_a = next_a;
        can_b = next_b;
    }
    out_line!(can_a || can_b);
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
