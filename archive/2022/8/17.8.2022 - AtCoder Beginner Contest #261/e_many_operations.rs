//{"name":"E - Many Operations","group":"AtCoder - AtCoder Beginner Contest 261","url":"https://atcoder.jp/contests/abc261/tasks/abc261_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 10\n3 3\n2 5\n1 12\n","output":"9\n15\n12\n"},{"input":"9 12\n1 1\n2 2\n3 3\n1 4\n2 5\n3 6\n1 7\n2 8\n3 9\n","output":"0\n2\n1\n0\n5\n3\n3\n11\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EManyOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let mut c = input.read_unsigned();

    let mut res = vec![2; 30];
    for _ in 0..n {
        let t = input.read_usize();
        let a = input.read_unsigned();
        for i in 0..30 {
            match t {
                1 => {
                    if !a.is_set(i) {
                        res[i] = 0;
                    }
                }
                2 => {
                    if a.is_set(i) {
                        res[i] = 3;
                    }
                }
                3 => {
                    if a.is_set(i) {
                        res[i] ^= 3;
                    }
                }
                _ => unreachable!(),
            }
            if c.is_set(i) {
                if !res[i].is_set(1) {
                    c.unset_bit(i);
                }
            } else {
                if res[i].is_set(0) {
                    c.set_bit(i);
                }
            }
        }
        out_line!(c);
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
