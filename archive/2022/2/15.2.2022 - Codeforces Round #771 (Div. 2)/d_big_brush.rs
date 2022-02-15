//{"name":"D. Big Brush","group":"Codeforces - Codeforces Round #771 (Div. 2)","url":"https://codeforces.com/contest/1638/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4 4\n5 5 3 3\n1 1 5 3\n2 2 5 4\n2 2 4 4\n","output":"6\n1 3 3\n3 3 4\n2 2 5\n1 1 5\n2 1 1\n3 1 2\n"},{"input":"3 4\n1 1 1 1\n2 2 3 1\n2 2 1 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBigBrush"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D8;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut a = input.read_table::<usize>(n, m);

    let mut ans = Vec::new();
    let mut q = VecDeque::new();
    fn check(a: &Arr2d<usize>, i: usize, j: usize) -> Option<usize> {
        let mut res = None;
        for r in i..i + 2 {
            for c in j..j + 2 {
                if a[(r, c)] != 0 {
                    match res {
                        None => {
                            res = Some(a[(r, c)]);
                        }
                        Some(col) => {
                            if a[(r, c)] != col {
                                return None;
                            }
                        }
                    }
                }
            }
        }
        res
    }
    for i in 0..n - 1 {
        for j in 0..m - 1 {
            if check(&a, i, j).is_some() {
                q.push_back((i, j));
            }
        }
    }
    while let Some((r, c)) = q.pop_front() {
        if let Some(col) = check(&a, r, c) {
            ans.push((r + 1, c + 1, col));
            for i in r..r + 2 {
                for j in c..c + 2 {
                    a[(i, j)] = 0;
                }
            }
            for (nr, nc) in D8::iter(r, c, n - 1, m - 1) {
                q.push_back((nr, nc));
            }
        }
    }
    if a.into_iter().filter(|&v| v != 0).count() != 0 {
        out_line!(-1);
        return;
    }
    ans.reverse();
    out_line!(ans.len());
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
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
