//{"name":"B - Sum-Product Ratio","group":"AtCoder - AtCoder Regular Contest 158","url":"https://atcoder.jp/contests/arc158/tasks/arc158_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n-2 -4 4 5\n","output":"-0.175000000000000\n-0.025000000000000\n"},{"input":"4\n1 1 1 1\n","output":"3.000000000000000\n3.000000000000000\n"},{"input":"5\n1 2 3 4 5\n","output":"0.200000000000000\n1.000000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSumProductRatio"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_size();
    let mut x = input.read_vec::<f64>(n);

    let mut poi = Vec::new();
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pos = x.lower_bound(&0.);
    let head = x.iter().take(pos).copied();
    if pos < 6 {
        poi.extend(head);
    } else {
        poi.extend(head.clone().take(3));
        poi.extend(head.rev().take(3));
    }
    let tail = x.iter().skip(pos).copied();
    if n - pos < 6 {
        poi.extend(tail);
    } else {
        poi.extend(tail.clone().take(3));
        poi.extend(tail.rev().take(3));
    }

    let mut min = None;
    let mut max = None;
    for i in 0..poi.len() {
        for j in 0..i {
            for k in 0..j {
                let val = (poi[i] + poi[j] + poi[k]) / (poi[i] * poi[j] * poi[k]);
                min.minim(val);
                max.maxim(val);
            }
        }
    }
    out_line!(min);
    out_line!(max);
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
