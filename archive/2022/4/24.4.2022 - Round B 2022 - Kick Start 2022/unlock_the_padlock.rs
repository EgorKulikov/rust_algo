//{"name":"Unlock the Padlock","group":"Google Coding Competitions - Round B 2022 - Kick Start 2022","url":"https://codingcompetitions.withgoogle.com/kickstart/round/00000000008caa74/0000000000acef55","interactive":false,"timeLimit":30000,"tests":[{"input":"2\n6 2\n1 1 0 1 0 1\n6 2\n0 1 0 0 1 1\n","output":"Case #1: 3\nCase #2: 2\n"},{"input":"2\n6 10\n1 1 2 2 3 3\n6 10\n1 1 9 9 1 1\n","output":"Case #1: 3\nCase #2: 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"UnlockThePadlock"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let d = input.read_long();
    let v = input.read_long_vec(n);

    let mut ans = Arr3d::new(2, n + 1, n + 1, None);
    let mut rec =
        RecursiveFunction4::new(|f, side: usize, l: usize, r: usize, delta: i64| -> i64 {
            match ans[(side, l, r)] {
                Some(res) => res,
                None => {
                    let res = if l == r {
                        0
                    } else {
                        let left = (v[l] + delta) % d;
                        let left_val = left.min(d - left) + f.call(0, l + 1, r, delta + d - left);
                        let right = (v[r - 1] + delta) % d;
                        let right_val =
                            right.min(d - right) + f.call(1, l, r - 1, delta + d - right);
                        left_val.min(right_val)
                    };
                    ans[(side, l, r)] = Some(res);
                    res
                }
            }
        });
    out_line!(format!("Case #{}:", test_case), rec.call(0, 0, n, 0));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
