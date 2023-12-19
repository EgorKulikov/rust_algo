//{"name":"day14","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day14"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut lines = input.read_lines();

    {
        // part 1
        let mut ans = 0;
        for i in lines[0].indices() {
            let mut cur = 0;
            for j in lines.indices() {
                if lines[j][i] == b'#' {
                    cur = j + 1;
                } else if lines[j][i] == b'O' {
                    ans += lines.len() - cur;
                    cur += 1;
                }
            }
        }
        out.print_line(ans);
    }

    {
        // part 2
        let mut state = Vec::new();
        for i in 0..1000000000 {
            state.push(lines.clone());
            for i in lines[0].indices() {
                let mut cur = 0;
                for j in lines.indices() {
                    if lines[j][i] == b'#' {
                        cur = j + 1;
                    } else if lines[j][i] == b'O' {
                        lines[j][i] = b'.';
                        lines[cur][i] = b'O';
                        cur += 1;
                    }
                }
            }
            for j in lines.indices() {
                let mut cur = 0;
                for i in lines[0].indices() {
                    if lines[j][i] == b'#' {
                        cur = i + 1;
                    } else if lines[j][i] == b'O' {
                        lines[j][i] = b'.';
                        lines[j][cur] = b'O';
                        cur += 1;
                    }
                }
            }
            for i in lines[0].indices() {
                let mut cur = lines.len() - 1;
                for j in lines.indices().rev() {
                    if lines[j][i] == b'#' {
                        cur = j.max(1) - 1;
                    } else if lines[j][i] == b'O' {
                        lines[j][i] = b'.';
                        lines[cur][i] = b'O';
                        cur = cur.max(1) - 1;
                    }
                }
            }
            for j in lines.indices() {
                let mut cur = lines[0].len() - 1;
                for i in lines[0].indices().rev() {
                    if lines[j][i] == b'#' {
                        cur = i.max(1) - 1;
                    } else if lines[j][i] == b'O' {
                        lines[j][i] = b'.';
                        lines[j][cur] = b'O';
                        cur = cur.max(1) - 1;
                    }
                }
            }
            if let Some(pos) = state.iter().find_eq(&lines) {
                let detla = state.len() - pos;
                let cycles = 1000000000 - pos;
                lines = state[pos + cycles % detla].clone();
                break;
            }
        }
        let mut ans = 0;
        for i in lines[0].indices() {
            for j in lines.indices() {
                if lines[j][i] == b'O' {
                    ans += lines.len() - j;
                }
            }
        }
        out.print_line(ans);
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
