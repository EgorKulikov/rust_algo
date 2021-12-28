//{"name":"P4 - Reindeer Racing","group":"DMOJ - An Animal Contest 4","url":"https://dmoj.ca/problem/aac4p4","interactive":false,"timeLimit":1000,"tests":[{"input":"5 10 2 5 0 20\n1 2 3 4 5 5 4 3 2 1\n","output":"13\n1 0 0 0 0 0 0 0 1 0\n0 0 1 0 0 0 1 0 0 0\n0 0 1 0 1 0 0 1 0 1\n0 1 0 0 0 0 1 0 1 0\n0 0 0 1 0 1 0 0 0 0\n"},{"input":"5 5 1 5 22 25\n5 4 3 2 1\n","output":"-1\n"},{"input":"2 2 2 2 0 4\n1 1\n","output":"0\n0 0\n0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P4ReindeerRacing"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let s = input.read::<usize>() - 1;
    let f = input.read::<usize>() - 1;
    let l: usize = input.read();
    let r: usize = input.read();
    let k = input.read_vec::<usize>(m).dec_by_one();

    assert!(l <= r);
    assert!(r <= n * m);
    if s != f {
        if r == 0 || k.iter().find(&f).is_none() {
            out_line!(-1);
            return;
        }
    }
    let need_f = k.iter().rev().find(&f).unwrap_or(m);
    if l + need_f > n * m {
        out_line!(-1);
        return;
    }
    let mut ans = Arr2d::new(n, m, 0);
    let mut rem = l;
    if s != f {
        ans[(s, m - 1 - need_f)] = 1;
        if rem != 0 {
            rem -= 1;
        }
        for i in 0..n {
            if rem > 0 && ans[(i, m - 1 - need_f)] != 1 {
                rem -= 1;
                ans[(i, m - 1 - need_f)] = 1;
            }
        }
    }
    for i in 0..n {
        if i == f {
            continue;
        }
        for j in 0..m {
            if rem > 0 && ans[(i, j)] != 1 {
                rem -= 1;
                ans[(i, j)] = 1;
            }
        }
    }
    for i in 0..m {
        if rem > 0 && ans[(f, i)] != 1 {
            ans[(f, i)] = 1;
            rem -= 1;
        }
    }
    out_line!(ans.iter().sum::<i32>());
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
