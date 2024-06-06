//{"name":"F - Earn to Advance","group":"AtCoder - \tToyota Programming Contest 2024#3（AtCoder Beginner Contest 344）","url":"https://atcoder.jp/contests/abc344/tasks/abc344_f","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n1 2 3\n3 1 2\n2 1 1\n1 2\n4 3\n4 2\n1 5 7\n5 3 3\n","output":"8\n"},{"input":"3\n1 1 1\n1 1 1\n1 1 1\n1000000000 1000000000\n1000000000 1000000000\n1000000000 1000000000\n1000000000 1000000000 1000000000\n1000000000 1000000000 1000000000\n","output":"4000000004\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEarnToAdvance"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::md_arr::arr4d::Arr4d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::UpperDiv;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let p = input.read_long_table(n, n);
    let r = input.read_long_table(n, n - 1);
    let d = input.read_long_table(n - 1, n);

    let mut res = Arr4d::new(n, n, n, n, None);
    res[(0, 0, 0, 0)] = Some((0, Reverse(0)));
    for i in 0..n {
        for j in 0..n {
            for rr in i..n {
                for cc in j..n {
                    if let Some((moves, Reverse(rem))) = res[(i, j, rr, cc)] {
                        if p[(rr, cc)] > p[(i, j)] {
                            res[(rr, cc, rr, cc)].minim((moves, Reverse(rem)));
                        }
                        if rr + 1 < n {
                            let delta = d[(rr, cc)] - rem;
                            let add_moves = if delta > 0 {
                                delta.upper_div(p[(i, j)])
                            } else {
                                0
                            };
                            res[(i, j, rr + 1, cc)].minim((
                                moves + add_moves + 1,
                                Reverse(rem + add_moves * p[(i, j)] - d[(rr, cc)]),
                            ));
                        }
                        if cc + 1 < n {
                            let delta = r[(rr, cc)] - rem;
                            let add_moves = if delta > 0 {
                                delta.upper_div(p[(i, j)])
                            } else {
                                0
                            };
                            res[(i, j, rr, cc + 1)].minim((
                                moves + add_moves + 1,
                                Reverse(rem + add_moves * p[(i, j)] - r[(rr, cc)]),
                            ));
                        }
                    }
                }
            }
        }
    }
    let mut ans = None;
    for i in 0..n {
        for j in 0..n {
            if let Some((moves, _)) = res[(i, j, n - 1, n - 1)] {
                ans.minim(moves);
            }
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
