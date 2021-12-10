//{"name":"B - Fenwick Tree","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_b","interactive":false,"timeLimit":5000,"tests":[{"input":"5 5\n1 2 3 4 5\n1 0 5\n1 2 4\n0 3 10\n1 0 5\n1 0 3\n","output":"15\n7\n25\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFenwickTree"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let q = input.read();
    let a = input.read_vec::<u64>(n);

    let mut ft = FenwickTree::new(n);

    for (i, v) in a.into_iter().enumerate() {
        ft.add(i, v);
    }

    for _ in 0usize..q {
        let t: u8 = input.read();
        if t == 0 {
            let p = input.read();
            let x = input.read();
            ft.add(p, x);
        } else {
            let l = input.read();
            let r = input.read();
            out_line!(ft.get(l, r));
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
