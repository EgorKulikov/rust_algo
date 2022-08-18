//{"name":"C - Matrix Reducing","group":"AtCoder - freee Programming Contest 2022（AtCoder Beginner Contest 264）","url":"https://atcoder.jp/contests/abc264/tasks/abc264_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n1 2 3 4 5\n6 7 8 9 10\n11 12 13 14 15\n16 17 18 19 20\n2 3\n6 8 9\n16 18 19\n","output":"Yes\n"},{"input":"3 3\n1 1 1\n1 1 1\n1 1 1\n1 1\n2\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMatrixReducing"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let h1 = input.read_usize();
    let w1 = input.read_usize();
    let a = input.read_table::<i32>(h1, w1);
    let h2 = input.read_usize();
    let w2 = input.read_usize();
    let b = input.read_table::<i32>(h2, w2);

    output().bool_output = BoolOutput::YesNo;
    for i in 0i32..(1 << h1) {
        if i.count_ones().into_usize() != h2 {
            continue;
        }
        for j in 0i32..(1 << w1) {
            if j.count_ones().into_usize() != w2 {
                continue;
            }
            let mut good = true;
            let mut r = 0;
            for k in 0..h1 {
                if !i.is_set(k) {
                    continue;
                }
                let mut c = 0;
                for l in 0..w1 {
                    if !j.is_set(l) {
                        continue;
                    }
                    if b[(r, c)] != a[(k, l)] {
                        good = false;
                    }
                    c += 1;
                }
                r += 1;
            }
            if good {
                out_line!(true);
                return;
            }
        }
    }
    out_line!(false);
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
