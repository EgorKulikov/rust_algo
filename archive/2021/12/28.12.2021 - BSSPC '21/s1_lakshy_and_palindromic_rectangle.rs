//{"name":"S1 - Lakshy and Palindromic Rectangle","group":"DMOJ - BSSPC '21","url":"https://dmoj.ca/problem/bsspc21s1","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\niii\nooo\n...\n","output":"iii\nooo\niii\n"},{"input":"1 33\nccchasbecome........inrecentyears\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S1LakshyAndPalindromicRectangle"}}}

use algo_lib::collections::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut grid = input.read_table::<char>(n, m);

    for i in 0..=n / 2 {
        for j in 0..=m / 2 {
            let mut res = None;
            let mut update = |i, j| -> bool {
                let c = grid[(i, j)];
                if c != '.' {
                    match res {
                        Some(d) => {
                            if c != d {
                                out_line!(-1);
                                return true;
                            }
                        }
                        None => res = Some(c),
                    }
                }
                false
            };
            if update(i, j) {
                return;
            }
            if update(i, m - 1 - j) {
                return;
            }
            if update(n - 1 - i, j) {
                return;
            }
            if update(n - 1 - i, m - 1 - j) {
                return;
            }
            let c = match res {
                Some(c) => c,
                None => 'a',
            };
            grid[(i, j)] = c;
            grid[(i, m - j - 1)] = c;
            grid[(n - i - 1, j)] = c;
            grid[(n - i - 1, m - j - 1)] = c;
        }
    }
    output().print_table(&grid);
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
