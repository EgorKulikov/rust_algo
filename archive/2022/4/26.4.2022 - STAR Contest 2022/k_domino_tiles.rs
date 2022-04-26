//{"name":"K. Domino Tiles","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/K","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n6 2\n5 3\n2\n6 3\n5 2\n3\n3 0 4\n5 2 5\n6\n4 1 3 5 1 6\n1 3 1 6 3 4\n","output":"0\n1\n-1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KDominoTiles"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);
    let b = input.read_usize_vec(n);

    let sum_a: usize = a.iter().sum();
    let sum_b: usize = b.iter().sum();
    if sum_a % 2 != sum_b % 2 {
        out_line!(-1);
        return;
    }
    let mut delta = vec![0; 13];
    for (a, b) in a.into_iter().zip(b.into_iter()) {
        delta[6 + b - a] += 1;
    }
    let mut ans = vec![None; sum_a + sum_b + 1];
    ans[sum_a] = Some(0);

    for _ in 0..2 {
        for (i, &d) in delta.iter().skip(6).enumerate().skip(1) {
            for j in 0..i {
                let mut v = VecDeque::new();
                let mut shift = 0;
                for k in (j..ans.len()).step_by(i) {
                    if let Some(&(pos, _)) = v.front() {
                        if pos + i * d < k {
                            v.pop_front();
                        }
                    }
                    if let Some(a) = ans[k] {
                        while let Some(&(_, r)) = v.back() {
                            if r > shift - a {
                                break;
                            }
                            v.pop_back();
                        }
                        v.push_back((k, shift - a));
                    }
                    if let Some(&(_, r)) = v.front() {
                        ans[k].minim(shift - r);
                    }
                    shift += 1;
                }
            }
        }

        ans.reverse();
        delta.reverse();
    }

    out_line!(ans[(sum_a + sum_b) / 2]);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
