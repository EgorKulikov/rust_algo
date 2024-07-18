//{"name":"F - Negative Traveling Salesman","group":"AtCoder - AtCoder Beginner Contest 338","url":"https://atcoder.jp/contests/abc338/tasks/abc338_f","interactive":false,"timeLimit":6000,"tests":[{"input":"3 4\n1 2 5\n2 1 -3\n2 3 -4\n3 1 100\n","output":"-2\n"},{"input":"3 2\n1 2 0\n2 1 0\n","output":"No\n"},{"input":"5 9\n1 2 -246288\n4 5 -222742\n3 1 246288\n3 4 947824\n5 2 -178721\n4 3 -947824\n5 4 756570\n2 5 707902\n5 1 36781\n","output":"-449429\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FNegativeTravelingSalesman"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut cost = Arr2d::new(n, n, None);
    for (a, b, c) in edges {
        cost[(a, b)].minim(c);
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if let Some(d) = cost[(j, i)] {
                    if let Some(e) = cost[(i, k)] {
                        cost[(j, k)].minim(d + e);
                    }
                }
            }
        }
    }
    let mut ans = Arr2d::new(1 << n, n, None);
    for i in 0..n {
        ans[(1 << i, i)] = Some(0);
    }
    for mask in usize::iter_all(n) {
        if mask.count_ones() <= 1 {
            continue;
        }
        for i in 0..n {
            if !mask.is_set(i) {
                continue;
            }
            for j in 0..n {
                if !mask.is_set(j) || i == j {
                    continue;
                }
                if let Some(d) = cost[(j, i)] {
                    if let Some(c) = ans[(mask.without_bit(i), j)] {
                        ans[(mask, i)].minim(c + d);
                    }
                }
            }
        }
        for i in 0..n {
            if !mask.is_set(i) {
                continue;
            }
            for j in 0..n {
                if !mask.is_set(j) || i == j {
                    continue;
                }
                if let Some(d) = cost[(j, i)] {
                    if let Some(c) = ans[(mask, j)] {
                        ans[(mask, i)].minim(c + d);
                    }
                }
            }
        }
    }
    let mut res = None;
    let d = usize::all_bits(n);
    for i in 0..n {
        if let Some(c) = ans[(d, i)] {
            res.minim(c);
        }
    }
    out.set_bool_output(BoolOutput::YesNo);
    if let Some(d) = res {
        out.print_line(d);
    } else {
        out.print_line(false);
    }
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
                solve(&mut input, &mut output, i, &pre_calc);
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
