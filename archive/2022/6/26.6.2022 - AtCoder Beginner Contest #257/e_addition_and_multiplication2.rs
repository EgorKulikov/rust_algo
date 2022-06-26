//{"name":"E - Addition and Multiplication 2","group":"AtCoder - NS Solutions Corporation Programming Contest 2022（AtCoder Beginner Contest 257）","url":"https://atcoder.jp/contests/abc257/tasks/abc257_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 4 3 3 2 5 3 5 3\n","output":"95\n"},{"input":"20\n1 1 1 1 1 1 1 1 1\n","output":"99999999999999999999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAdditionAndMultiplication2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let c = input.read_usize_vec(9);

    let min = *c.iter().min().unwrap();
    let mut rem = n % min;
    for _ in 0..(n / min) {
        for j in (0..9).rev() {
            if c[j] - min <= rem {
                rem -= c[j] - min;
                out!(j + 1);
                break;
            }
        }
    }
    out_line!();
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
