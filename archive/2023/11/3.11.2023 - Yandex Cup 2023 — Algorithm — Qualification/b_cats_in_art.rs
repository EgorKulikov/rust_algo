//{"name":"B. Cats in Art","group":"Yandex - Yandex Cup 2023 — Algorithm — Qualification","url":"https://contest.yandex.com/contest/54452/problems/B/","interactive":false,"timeLimit":1000,"tests":[{"input":"5 4\n1 1 10 1 1\n","output":"5\n"},{"input":"6 3\n3 10 5 3 5 5\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCatsInArt"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let c = input.read_size_vec(n);

    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by_key(|&i| c[i]);
    let target = c[order[m - 1]];
    let mut at_target = Vec::new();
    let mut left = n;
    let mut right = 0;
    let mut spread = m - 1;
    for i in 0..n {
        if c[i] == target {
            at_target.push(i);
        } else if c[i] < target {
            left.minim(i);
            right.maxim(i);
            spread -= 1;
        }
    }
    let mut ans = None;
    for i in spread..at_target.len() {
        ans.minim(at_target[i].max(right) - at_target[i - spread].min(left) + 1);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
