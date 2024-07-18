//{"name":"D - Island Tour","group":"AtCoder - AtCoder Beginner Contest 338","url":"https://atcoder.jp/contests/abc338/tasks/abc338_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 3 2\n","output":"2\n"},{"input":"4 5\n2 4 2 4 2\n","output":"8\n"},{"input":"163054 10\n62874 19143 77750 111403 29327 56303 6659 18896 64175 26369\n","output":"390009\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DIslandTour"}}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let x = input.read_size_vec(m).dec();

    let mut delta = vec![0; 2 * n];
    let mut add = 0;
    for (&i, &j) in x.consecutive_iter() {
        let dist = i.abs_diff(j);
        if 2 * dist <= n {
            let d = (n - 2 * dist) as i64;
            delta[i.min(j)] += d;
            delta[i.max(j)] -= d;
            add += dist as i64;
        } else {
            let d = (2 * dist - n) as i64;
            delta[i.max(j)] += d;
            delta[i.min(j) + n] -= d;
            add += (n - dist) as i64;
        }
    }
    let mut cur = 0;
    let mut d = Vec::with_capacity(2 * n);
    for i in 0..2 * n {
        cur += delta[i];
        d.push(cur);
    }
    let ans = (0..n).map(|i| d[i] + d[i + n]).min().unwrap();
    out.print_line(ans + add);
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
