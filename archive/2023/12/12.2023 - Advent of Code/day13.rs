//{"name":"day13","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day13"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let lines = input.read_lines();
    let pat = lines.split(|s| s.is_empty()).collect_vec();

    let solve = |target: usize| -> usize {
        let mut ans = 0;
        for pat in &pat {
            let mut cur = None;
            for i in 1..pat.len() {
                let mut err = 0;
                for j in i..pat.len().min(2 * i) {
                    for k in 0..pat[0].len() {
                        if pat[j][k] != pat[2 * i - 1 - j][k] {
                            err += 1;
                        }
                    }
                }
                if err == target {
                    cur = Some(i * 100);
                    break;
                }
            }
            for i in 1..pat[0].len() {
                let mut err = 0;
                for j in i..pat[0].len().min(2 * i) {
                    for k in 0..pat.len() {
                        if pat[k][j] != pat[k][2 * i - 1 - j] {
                            err += 1;
                        }
                    }
                }
                if err == target {
                    cur = Some(i);
                    break;
                }
            }
            ans += cur.unwrap();
        }
        ans
    };

    {
        //part 1
        out.print_line(solve(0));
    }

    {
        //part 2
        out.print_line(solve(1));
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
