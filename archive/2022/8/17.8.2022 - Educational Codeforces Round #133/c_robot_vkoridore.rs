//{"name":"C. Робот в коридоре","group":"Codeforces - Educational Codeforces Round 133 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1716/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 0 1\n4 3 2\n5\n0 4 8 12 16\n2 6 10 14 18\n4\n0 10 10 10\n10 10 10 10\n2\n0 0\n0 0\n","output":"5\n19\n17\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRobotVKoridore"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_table::<usize>(2, n);

    for i in 0..n {
        for j in 0..2 {
            if i != 0 || j != 0 {
                a[(j, i)] += 1;
            }
        }
    }
    let mut end = Arr2d::new(2, n, 0);
    for i in 0..2 {
        if a[(i, n - 1)] > a[(1 - i, n - 1)] {
            end[(i, n - 1)] = a[(i, n - 1)];
        } else {
            end[(i, n - 1)] = a[(1 - i, n - 1)] + 1;
        }
    }
    for i in (0..n - 1).rev() {
        let delta = (n - 1 - i) * 2;
        for j in 0..2 {
            end[(j, i)] = (a[(j, i)])
                .max(end[(j, i + 1)] + 1)
                .max(a[(1 - j, i)] + delta + 1);
        }
    }
    let mut ans = None;
    let mut min_ans = 2 * n - 1;
    for i in 0..n {
        let j = i % 2;
        if i != 0 {
            min_ans.maxim(a[(j, i)] + 2 * (n - 1 - i) + 1);
        }
        ans.minim(min_ans.max(end[(1 - j, i)]));
        min_ans.maxim(a[(1 - j, i)] + 2 * (n - 1 - i));
    }
    out_line!(ans);
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
