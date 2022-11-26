//{"name":"F - BOX","group":"AtCoder - TOYOTA SYSTEMS Programming Contest 2022(AtCoder Beginner Contest 279)","url":"https://atcoder.jp/contests/abc279/tasks/abc279_f","interactive":false,"timeLimit":2000,"tests":[{"input":"5 10\n3 5\n1 1 4\n2 1\n2 4\n3 7\n1 3 1\n3 4\n1 1 4\n3 7\n3 6\n","output":"5\n4\n3\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBOX"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let q = input.read_usize();

    let mut set_id = (0..n).collect_vec();
    let mut ball_id = (0..n).collect_vec();
    let mut sets = (0..n).map(|i| (i, vec![i])).collect_vec();
    let mut next_set = n;
    let mut next_ball = n;
    for _ in 0..q {
        let tp = input.read_usize();
        match tp {
            1 => {
                let x = input.read_usize() - 1;
                let y = input.read_usize() - 1;
                let mut y_set_id = set_id[y];
                set_id[y] = next_set;
                next_set += 1;
                sets.push((y, vec![]));
                let mut x_set_id = set_id[x];
                if sets[y_set_id].1.len() > sets[set_id[x]].1.len() {
                    sets[y_set_id].0 = x;
                    set_id[x] = y_set_id;
                    swap(&mut x_set_id, &mut y_set_id);
                }
                let mut balls = Vec::new();
                swap(&mut balls, &mut sets[y_set_id].1);
                for i in balls {
                    ball_id[i] = x_set_id;
                    sets[x_set_id].1.push(i);
                }
            }
            2 => {
                let x = input.read_usize() - 1;
                sets[set_id[x]].1.push(next_ball);
                ball_id.push(set_id[x]);
                next_ball += 1;
            }
            3 => {
                let x = input.read_usize() - 1;
                out_line!(sets[ball_id[x]].0 + 1);
            }
            _ => unreachable!(),
        }
    }
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
