//{"name":"B. Магазин целых чисел","group":"Codeforces - Hello 2022","url":"https://codeforces.com/contest/1621/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n2 4 20\n7 8 22\n2\n5 11 42\n5 11 42\n6\n1 4 4\n5 8 9\n7 8 7\n2 10 252\n1 11 271\n1 10 1\n","output":"20\n42\n42\n42\n4\n13\n11\n256\n271\n271\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMagazinTselikhChisel"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let segs: Vec<(u32, u32, u32)> = input.read_vec(n);

    let mut left_cost = None;
    let mut right_cost = None;
    let mut left = None;
    let mut right = None;
    let mut both_cost = None;
    for (l, r, c) in segs {
        if left.minim(l) {
            left = Some(l);
            left_cost = None;
            both_cost = None;
        }
        if right.maxim(r) {
            right = Some(r);
            right_cost = None;
            both_cost = None;
        }
        if left == Some(l) {
            left_cost.minim(c);
            if right == Some(r) {
                both_cost.minim(c);
            }
        }
        if right == Some(r) {
            right_cost.minim(c);
        }
        let ans = match both_cost {
            None => left_cost.unwrap() + right_cost.unwrap(),
            Some(both) => both.min(left_cost.unwrap() + right_cost.unwrap()),
        };
        out_line!(ans);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
