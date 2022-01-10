//{"name":"#1 - Lampice","group":"DMOJ - COCI '21 Contest 3","url":"https://dmoj.ca/problem/coci21c3p1","interactive":false,"timeLimit":1000,"tests":[{"input":"8 6\n10 1 1 1 1 1 1 5\n","output":"1\n1\n"},{"input":"3 2\n1 2 1\n","output":"-1\n"},{"input":"10 2\n1 5 1 5 2 5 6 2 5 6\n","output":"2\n1 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Lampice"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_int_vec(n);
    for i in 0..n {
        for j in 1..=(n - i) / k {
            let mut good = true;
            for l in i + j..i + j * k {
                if a[l] != a[l - j] {
                    good = false;
                    break;
                }
            }
            if good {
                out_line!(j);
                out_line!(&a[i..i + j]);
                return;
            }
        }
    }
    out_line!(-1);
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
