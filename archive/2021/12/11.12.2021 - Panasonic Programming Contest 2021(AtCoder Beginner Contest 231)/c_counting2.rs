//{"name":"C - Counting 2","group":"AtCoder - Panasonic Programming Contest 2021(AtCoder Beginner Contest 231)","url":"https://atcoder.jp/contests/abc231/tasks/abc231_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1\n100 160 130\n120\n","output":"2\n"},{"input":"5 5\n1 2 3 4 5\n6\n5\n4\n3\n2\n","output":"0\n1\n2\n3\n4\n"},{"input":"5 5\n804289384 846930887 681692778 714636916 957747794\n424238336\n719885387\n649760493\n596516650\n189641422\n","output":"5\n3\n5\n5\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCounting2"}}}

use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let q: usize = input.read();
    let mut a: Vec<u32> = input.read_vec(n);

    a.sort();
    for _ in 0..q {
        let x: u32 = input.read();
        out_line!(n - a.as_slice().lower_bound(&x));
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
