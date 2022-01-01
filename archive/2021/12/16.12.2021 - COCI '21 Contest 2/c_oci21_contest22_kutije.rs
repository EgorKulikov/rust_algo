//{"name":"COCI '21 Contest 2 #2 Kutije","group":"DMOJ","url":"https://dmoj.ca/problem/coci21c2p2","interactive":false,"timeLimit":1000,"tests":[{"input":"4 1 3\n1 2 4 3\n1 1\n1 2\n3 4\n","output":"DA\nNE\nDA\n"},{"input":"4 2 4\n2 1 3 4\n1 2 4 3\n2 1\n3 4\n1 4\n2 3\n","output":"DA\nDA\nNE\nNE\n"},{"input":"6 2 2\n2 1 4 5 3 6\n3 2 4 1 5 6\n1 5\n6 3\n","output":"DA\nNE\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COCI21Contest22Kutije"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let q = input.read();

    let mut dsu = DSU::new(n);
    for _ in 0..m {
        let p = input.read_vec::<usize>(n).dec_by_one();
        for i in 0..n {
            dsu.join(i, p[i]);
        }
    }
    for _ in 0..q {
        let a = input.read::<usize>() - 1;
        let b = input.read::<usize>() - 1;
        out_line!(if dsu.get(a) == dsu.get(b) { "DA" } else { "NE" });
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
