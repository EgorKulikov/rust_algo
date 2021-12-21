//{"name":"P2 - Cookie Sprinkler","group":"DMOJ - SAC '22 Code Challenge 2","url":"https://dmoj.ca/problem/sac22cc2p2","interactive":false,"timeLimit":1000,"tests":[{"input":"6 6\n1 2 3\n1 5 6\n2 2 3 5 6\n1 4 4\n2 2 3 4 4\n2 2 3 5 6\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P2CookieSprinkler"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let q = input.read();

    let mut cookies = Arr2d::new(n, n, 0);
    let mut ans = 0;
    for _ in 0..q {
        let t: u8 = input.read();
        if t == 1 {
            let x = input.read::<usize>() - 1;
            let y = input.read::<usize>() - 1;
            cookies[(x, y)] += 1;
        } else {
            let x1 = input.read::<usize>() - 1;
            let y1 = input.read::<usize>() - 1;
            let x2 = input.read();
            let y2 = input.read();
            for i in x1..x2 {
                for j in y1..y2 {
                    ans += cookies[(i, j)];
                }
            }
        }
    }
    out_line!(ans);
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
