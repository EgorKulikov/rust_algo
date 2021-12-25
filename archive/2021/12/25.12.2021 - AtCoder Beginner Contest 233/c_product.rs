//{"name":"C - Product","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_c","interactive":false,"timeLimit":2000,"tests":[{"input":"2 40\n3 1 8 4\n2 10 5\n","output":"2\n"},{"input":"3 200\n3 10 10 10\n3 10 10 10\n5 2 2 2 2 2\n","output":"45\n"},{"input":"3 1000000000000000000\n2 1000000000 1000000000\n2 1000000000 1000000000\n2 1000000000 1000000000\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CProduct"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n: usize = input.read();
    let x: u128 = input.read();
    let mut a = Vec::new();
    for _ in 0..n {
        a.push(input.read::<Vec<u128>>());
    }

    let mut rec = RecursiveFunction2::new(|f, step: usize, val: u128| -> u32 {
        if val > x {
            0
        } else if step == n {
            if val == x {
                1
            } else {
                0
            }
        } else {
            let mut res = 0;
            for i in a[step].iter().cloned() {
                res += f.call(step + 1, val * i);
            }
            res
        }
    });
    out_line!(rec.call(0, 1));
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
