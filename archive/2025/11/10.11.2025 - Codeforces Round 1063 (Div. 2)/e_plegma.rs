//{"name":"E. Plegma","group":"Codeforces - Codeforces Round 1063 (Div. 2)","url":"https://codeforces.com/contest/2163/problem/E","interactive":false,"timeLimit":5000,"tests":[{"input":"first\n2\n2 1\n11\n10\n2 0\n10\n01\n","output":"2 2\n2 1\n"},{"input":"second\n2\n2\n10\n10\n2\n01\n10\n","output":"1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mode = input.read_str();
    let t = input.read_size();
    for _ in 0..t {
        if mode.as_slice() == b"first" {
            let n = input.read_size();
            let c = input.read_int();
            let g = input.read_char_table(n, n);

            if c == 0 {
                let mut zero_col = n;
                for i in 0..n {
                    if g[(0, i)] == b'0' {
                        zero_col = i;
                        break;
                    }
                }
                if zero_col == n {
                    let mut non_one_col = n;
                    for (i, j) in g.indices() {
                        if g[(i, j)] == b'0' {
                            non_one_col = j;
                            break;
                        }
                    }
                    assert_ne!(non_one_col, n);
                    out.print_line((1, non_one_col + 1));
                } else {
                    let mut one_row = n;
                    for i in 0..n {
                        if g[(i, 0)] == b'1' {
                            one_row = i;
                            break;
                        }
                    }
                    if one_row == n {
                        let mut non_zero_row = n;
                        for (i, j) in g.indices() {
                            if g[(i, j)] == b'1' {
                                non_zero_row = i;
                                break;
                            }
                        }
                        assert_ne!(non_zero_row, n);
                        out.print_line((non_zero_row + 1, 1));
                    } else {
                        out.print_line((one_row + 1, zero_col + 1));
                    }
                }
            } else {
                let mut one_col = n;
                for i in 0..n {
                    if g[(0, i)] == b'1' {
                        one_col = i;
                        break;
                    }
                }
                if one_col == n {
                    out.print_line((1, 1));
                } else {
                    let mut zero_row = n;
                    for i in 0..n {
                        if g[(i, 0)] == b'0' {
                            zero_row = i;
                            break;
                        }
                    }
                    if zero_row == n {
                        out.print_line((1, 1));
                    } else {
                        out.print_line((zero_row + 1, one_col + 1));
                    }
                }
            }
            out.flush();
        } else {
            let _n = input.read_size();
            let r = input.read_str();
            let c = input.read_str();

            out.print_line((r <= c) as u32);
            out.flush();
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::RunTwice;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
