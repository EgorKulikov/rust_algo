//{"name":"ucup20_f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup20_f"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut s = input.read_str();

    enum ParseType {
        Expr,
        Term,
        Factor,
    }
    let mut pos = 0;
    s += b')';
    let mut rec = RecursiveFunction::new(|f, tp: ParseType| -> (i64, i64, bool) {
        match tp {
            ParseType::Expr => {
                let mut n = 0;
                let mut log = 0;
                let mut log_log = false;
                loop {
                    let (c_n, c_log, c_log_log) = f.call(ParseType::Term);
                    if n.maxim(c_n) {
                        log = c_log;
                        log_log = c_log_log;
                    } else if n == c_n && log.maxim(c_log) {
                        log_log = c_log_log;
                    } else if n == c_n && log == c_log && c_log_log {
                        log_log = true;
                    }
                    if s[pos] == b'+' {
                        pos += 1;
                    } else {
                        return (n, log, log_log);
                    }
                }
            }
            ParseType::Term => {
                let mut n = 0;
                let mut log = 0;
                let mut log_log = false;
                loop {
                    let (c_n, c_log, c_log_log) = f.call(ParseType::Factor);
                    n += c_n;
                    log += c_log;
                    log_log |= c_log_log;
                    if s[pos] == b'*' {
                        pos += 1;
                    } else {
                        return (n, log, log_log);
                    }
                }
            }
            ParseType::Factor => {
                let (n, log, log_log) = match s[pos] {
                    b'(' => {
                        pos += 1;
                        let (n, log, log_log) = f.call(ParseType::Expr);
                        assert_eq!(s[pos], b')');
                        pos += 1;
                        (n, log, log_log)
                    }
                    b'N' => {
                        pos += 1;
                        (1, 0, false)
                    }
                    b'l' => {
                        pos += 4;
                        let (n, log, log_log) = f.call(ParseType::Expr);
                        assert_eq!(s[pos], b')');
                        pos += 1;
                        if n != 0 {
                            (0, 1, false)
                        } else if log != 0 || log_log {
                            (0, 0, true)
                        } else {
                            (0, 0, false)
                        }
                    }
                    _ => unreachable!(),
                };
                let power = if s[pos] == b'^' {
                    pos += 1;
                    let mut power = 0;
                    while s[pos].is_ascii_digit() {
                        power *= 10;
                        power += (s[pos] - b'0') as i64;
                        pos += 1;
                    }
                    power
                } else {
                    1
                };
                (n * power, log * power, log_log)
            }
        }
    });
    let (n, mut log, log_log) = rec.call(ParseType::Expr);
    if log_log {
        log += 1;
    }
    out.print_line((n, log));
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
