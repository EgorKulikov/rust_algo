//{"name":"M. NCQ","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/M","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 5\n1 7\n","output":"8\n"},{"input":"1\n0 0\n","output":"0\n"},{"input":"2\n-2 1\n-2 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MNCQ"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Detuple;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut requests = input.read_int_pair_vec(n);

    requests.sort();
    let (x, t) = requests.detuple();
    let mut left = Arr2d::new(n, n, i32::MAX / 2);
    let mut right = Arr2d::new(n, n, i32::MAX / 2);
    left[(0, n - 1)] = t[0].max(x[0].abs());
    right[(0, n - 1)] = t[n - 1].max(x[n - 1].abs());
    for i in 0..n {
        for j in (i + 1..n).rev() {
            let cand = (left[(i, j)] + (x[i + 1] - x[i])).max(t[i + 1]);
            left[(i + 1, j)].minim(cand);
            let cand = (left[(i, j)] + (x[j] - x[i])).max(t[j]);
            right[(i + 1, j)].minim(cand);
            let cand = (right[(i, j)] + (x[j] - x[i])).max(t[i]);
            left[(i, j - 1)].minim(cand);
            let cand = (right[(i, j)] + (x[j] - x[j - 1])).max(t[j - 1]);
            right[(i, j - 1)].minim(cand);
        }
    }
    let mut res = i32::MAX / 2;
    for i in 0..n {
        res.minim(left[(i, i)]);
        res.minim(right[(i, i)]);
    }
    out_line!(res);
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
