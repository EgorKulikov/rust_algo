//{"name":"I, O Bot","group":"Google Coding Competitions - Round 2 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008778ec/0000000000b15167","interactive":false,"timeLimit":40000,"tests":[{"input":"4\n5 0\n3 0\n6 0\n8 0\n10 1\n15 1\n5 10\n3 0\n6 0\n8 0\n10 1\n15 1\n5 1\n3 0\n6 0\n8 0\n10 1\n15 1\n2 0\n1000000000 0\n-1000000000 1\n","output":"Case #1: 52\nCase #2: 56\nCase #3: 54\nCase #4: 4000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IOBot"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let c = input.read_long();
    let balls = input.read_long_pair_vec(n);

    let mut pos0 = Vec::new();
    let mut pos1 = Vec::new();
    let mut neg0 = Vec::new();
    let mut neg1 = Vec::new();
    for (x, s) in balls {
        if x > 0 {
            if s == 0 {
                pos0.push(x);
            } else {
                pos1.push(x);
            }
        } else {
            if s == 0 {
                neg0.push(-x);
            } else {
                neg1.push(-x);
            }
        }
    }
    let mut ans = 0;
    for (mut one, mut zero) in vec![(pos1, pos0), (neg1, neg0)] {
        one.sort_unstable();
        zero.sort_unstable();
        let n = one.len();
        let m = zero.len();
        let mut res = Arr2d::new(n + 1, m + 1, std::i64::MAX);
        res[(n, m)] = 0;
        for i in (0..=n).rev() {
            for j in (0..=m).rev() {
                if i > 0 {
                    if j > 0 {
                        let cand = res[(i, j)] + one[i - 1].max(zero[j - 1]) * 2;
                        res[(i - 1, j - 1)].minim(cand);
                    }
                    let cand = res[(i, j)] + 2 * one[i - 1];
                    res[(i - 1, j)].minim(cand);
                    if i > 1 {
                        let cand = res[(i, j)] + 2 * one[i - 1] + c;
                        res[(i - 2, j)].minim(cand);
                    }
                }
                if j > 0 {
                    let cand = res[(i, j)] + 2 * zero[j - 1];
                    res[(i, j - 1)].minim(cand);
                    if j > 1 {
                        let cand = res[(i, j)] + 2 * zero[j - 1] + c;
                        res[(i, j - 2)].minim(cand);
                    }
                }
            }
        }
        ans += res[(0, 0)];
    }
    out_line!(format!("Case #{}:", test_case), ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
