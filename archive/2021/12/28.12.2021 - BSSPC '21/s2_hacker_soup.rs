//{"name":"S2 - Hacker Soup","group":"DMOJ - BSSPC '21","url":"https://dmoj.ca/problem/bsspc21s2","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2 2\n1 1 3 3\n2 2 3 3\n3 2\n1 2\n","output":"3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S2HackerSoup"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let q = input.read_usize();
    let ops: Vec<(usize, usize, usize, usize)> = input.read_vec(k);

    for _ in 0..q {
        let mut r = input.read_usize();
        let mut c = input.read_usize();
        for (from_r, from_c, to_r, to_c) in ops.iter().cloned().rev() {
            if r < from_r || r > to_r || c < from_c || c > to_c {
                continue;
            }
            let dr = r - from_r;
            let dc = c - from_c;
            let side = to_r - from_r;
            r = from_r + (side - dc);
            c = from_c + dr;
        }
        out_line!((r - 1) * n + c);
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
