//{"name":"C - Filling 3x3 array","group":"AtCoder - Tokio Marine & Nichido Fire Insurance Programming Contest 2022（AtCoder Beginner Contest 256)","url":"https://atcoder.jp/contests/abc256/tasks/abc256_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4 6 3 3 7\n","output":"1\n"},{"input":"3 4 5 6 7 8\n","output":"0\n"},{"input":"5 13 10 6 13 9\n","output":"120\n"},{"input":"20 25 30 22 29 24\n","output":"30613\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFilling3x3Array"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let h = input.read_usize_vec(3);
    let w = input.read_usize_vec(3);

    if h.iter().sum::<usize>() != w.iter().sum::<usize>() {
        out_line!(0);
        return;
    }
    let mut grid = Arr2d::new(3, 3, 0);
    let mut ans = 0;
    for i in 1..=28 {
        for j in 1..=28 {
            for k in 1..=28 {
                for l in 1..=28 {
                    grid[(0, 0)] = i;
                    grid[(0, 1)] = j;
                    grid[(1, 0)] = k;
                    grid[(1, 1)] = l;
                    let mut good = true;
                    for a in 0..3 {
                        for b in 0..3 {
                            if a == 2 {
                                if grid[(0, b)] + grid[(1, b)] >= w[b] {
                                    good = false;
                                    break;
                                }
                                grid[(2, b)] = w[b] - (grid[(0, b)] + grid[(1, b)]);
                            }
                            if b == 2 {
                                if grid[(a, 0)] + grid[(a, 1)] >= h[a] {
                                    good = false;
                                    break;
                                }
                                grid[(a, 2)] = h[a] - (grid[(a, 0)] + grid[(a, 1)]);
                            }
                        }
                        if !good {
                            break;
                        }
                    }
                    if good {
                        ans += 1;
                    }
                }
            }
        }
    }
    out_line!(ans);
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
