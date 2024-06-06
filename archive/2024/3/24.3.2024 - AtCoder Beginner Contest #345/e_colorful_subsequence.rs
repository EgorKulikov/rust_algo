//{"name":"E - Colorful Subsequence","group":"AtCoder - Monoxer Programming Contest 2024（AtCoder Beginner Contest 345）","url":"https://atcoder.jp/contests/abc345/tasks/abc345_e","interactive":false,"timeLimit":5000,"tests":[{"input":"5 2\n1 1\n3 5\n3 3\n1 4\n1 2\n","output":"10\n"},{"input":"3 1\n1 10\n1 10\n1 10\n","output":"-1\n"},{"input":"3 1\n1 1\n2 2\n3 3\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EColorfulSubsequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let balls = input.read_vec::<(usize, i64)>(n);

    let mut cur = vec![(i64::MIN, 0, i64::MIN); k + 1];
    cur[0].0 = 0;
    let mut next = vec![(i64::MIN, 0, i64::MIN); k + 1];

    for step in (0..n).rev() {
        let (cur_color, cur_val) = balls[step];
        for rem_skip in 0..=k {
            let (best, color, second) = cur[rem_skip];
            let mut best = if color != cur_color { best } else { second } + cur_val;
            let mut color = cur_color;
            let mut second = i64::MIN;
            if rem_skip != 0 {
                let (best2, color2, second2) = cur[rem_skip - 1];
                if best2 > best {
                    if color2 != color {
                        second = best.max(second2);
                    } else {
                        second = second2;
                    }
                    best = best2;
                    color = color2;
                } else if color2 != cur_color {
                    second = best2;
                } else {
                    second = second2;
                }
            }
            next[rem_skip] = (best, color, second);
        }
        std::mem::swap(&mut cur, &mut next);
    }

    let mut ans = cur[k].0;
    if ans < 0 {
        ans = -1;
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
