//{"name":"P5 - Even Colouring","group":"DMOJ - SAC '22 Code Challenge 2","url":"https://dmoj.ca/problem/sac22cc2p5","interactive":false,"timeLimit":1000,"tests":[{"input":"5 5\n1 5 6 9 3\n2 1 5\n2 2 5\n1 2 -4\n2 1 5\n2 2 5\n","output":"10\n14\n10\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5EvenColouring"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let q = input.read();
    let a: Vec<i64> = input.read_vec(n);

    let mut even = FenwickTree::new(n);
    let mut odd = FenwickTree::new(n);
    for (i, v) in a.into_iter().enumerate() {
        if i % 2 == 0 {
            even.add(i, v);
        } else {
            odd.add(i, v);
        }
    }

    for _ in 0..q {
        let t: u8 = input.read();
        if t == 1 {
            let i = input.read::<usize>() - 1;
            let v: i64 = input.read();
            if i % 2 == 0 {
                let v = v - even.get(i, i + 1);
                even.add(i, v);
            } else {
                let v = v - odd.get(i, i + 1);
                odd.add(i, v);
            }
        } else {
            let l = input.read::<usize>() - 1;
            let r = input.read();
            if l % 2 == 0 {
                out_line!(even.get(l, r));
            } else {
                out_line!(odd.get(l, r));
            }
        }
    }
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
