//{"name":"F - Sugar Water 2","group":"AtCoder - AtCoder Beginner Contest 294","url":"https://atcoder.jp/contests/abc294/tasks/abc294_f","interactive":false,"timeLimit":4000,"tests":[{"input":"3 1 1\n1 2\n4 1\n1 4\n1 4\n","output":"50.000000000000000\n"},{"input":"2 2 2\n6 4\n10 1\n5 8\n9 6\n","output":"62.500000000000000\n"},{"input":"4 5 10\n5 4\n1 6\n7 4\n9 8\n2 2\n5 6\n6 7\n5 3\n8 1\n","output":"54.166666666666664\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSugarWater2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let ab = input.read_size_pair_vec(n);
    let cd = input.read_size_pair_vec(m);

    let mut left = 0.;
    let mut right = 1.;
    let mut ab_vals = vec![0.; n];
    let mut cd_vals = vec![0.; m];
    for _ in 0..100 {
        let mid = (left + right) / 2.;
        for i in 0..n {
            ab_vals[i] = (ab[i].0 as f64) * (1. - mid) - mid * (ab[i].1 as f64);
        }
        for i in 0..m {
            cd_vals[i] = (cd[i].0 as f64) * (1. - mid) - mid * (cd[i].1 as f64);
        }
        ab_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        cd_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut cur = 0;
        let mut at = m;
        for &ab in &ab_vals {
            while at > 0 && cd_vals[at - 1] + ab > 0. {
                at -= 1;
            }
            cur += m - at;
        }
        if cur >= k {
            left = mid;
        } else {
            right = mid;
        }
    }
    out_line!(left * 100.);
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
