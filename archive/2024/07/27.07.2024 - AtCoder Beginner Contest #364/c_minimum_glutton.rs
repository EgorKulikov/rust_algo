//{"name":"C - Minimum Glutton","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2024#2 (AtCoder Beginner Contest 364)","url":"https://atcoder.jp/contests/abc364/tasks/abc364_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 7 18\n2 3 5 1\n8 8 1 4\n","output":"2\n"},{"input":"5 200000000000000 200000000000000\n1 1 1 1 1\n2 2 2 2 2\n","output":"5\n"},{"input":"8 30 30\n1 2 3 4 5 6 7 8\n8 7 6 5 4 3 2 1\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMinimumGlutton"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let y = input.read_long();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut ans = n;
    for (mut c, x) in [(a, x), (b, y)] {
        c.sort();
        c.reverse();
        let mut total = 0;
        for (id, i) in c.into_iter().enumerate() {
            total += i;
            if total > x {
                ans.minim(id + 1);
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
}
//END MAIN
