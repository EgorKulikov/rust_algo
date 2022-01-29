//{"name":"D - Concatenate Subsequences","group":"AtCoder - AtCoder Regular Contest 134","url":"https://atcoder.jp/contests/arc134/tasks/arc134_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 1 3 1 2 2\n","output":"1 2\n"},{"input":"10\n38 38 80 62 62 67 38 78 74 52 53 77 59 83 74 63 80 61 68 55\n","output":"38 38 38 52 53 77 80 55\n"},{"input":"12\n52 73 49 63 55 74 35 68 22 22 74 50 71 60 52 62 65 54 70 59 65 54 60 52\n","output":"22 22 50 65 54 52\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DConcatenateSubsequences"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_utils::factorial;
use algo_lib::{out_line, zip};

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_usize_vec(2 * n);

    let min = *a.iter().take(n).min().unwrap();
    let min_tail = a
        .iter()
        .take(n)
        .zip(a.iter().skip(n))
        .filter_map(|(&a, &b)| if a == min { Some(b) } else { None })
        .min()
        .unwrap();
    if min_tail <= min {
        out_line!(min, min_tail);
        return;
    }
    let first_tail = a
        .iter()
        .take(n)
        .zip(a.iter().skip(n))
        .filter_map(|(&a, &b)| if a == min { Some(b) } else { None })
        .next()
        .unwrap();
    let mut next_min = Vec::with_capacity(n);
    for &i in a.iter().take(n).rev() {
        if next_min.is_empty() {
            next_min.push(i);
        } else {
            next_min.push(i.min(*next_min.last().unwrap()));
        }
    }
    next_min.reverse();
    let mut front = Vec::new();
    let mut tail = Vec::new();
    let mut more = false;
    let mut less = false;
    for (&i, &j, &k) in zip!(&next_min, a.iter().take(n), a.iter().skip(n)) {
        if i > first_tail || i == first_tail && !more {
            break;
        }
        if i == j {
            front.push(j);
            tail.push(k);
            if !more && !less && k != first_tail {
                if k < first_tail {
                    less = true;
                } else {
                    more = true;
                }
            }
        }
    }
    let ans = front.into_iter().chain(tail.into_iter()).collect_vec();
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
