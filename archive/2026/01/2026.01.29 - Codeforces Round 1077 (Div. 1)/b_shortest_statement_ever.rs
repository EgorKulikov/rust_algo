//{"name":"B. Shortest Statement Ever","group":"Codeforces - Codeforces Round 1077 (Div. 1)","url":"https://codeforces.com/contest/2187/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n0 0\n1 1\n3 6\n7 11\n4 4\n123 321\n1073741823 1073741822\n","output":"0 0\n2 1\n3 8\n6 9\n4 3\n128 321\n1073741824 1073741822\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization3;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;
use std::cmp::Ordering;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_long();
    let y = input.read_long();

    let mut ans = None;
    for expected_x in [Ordering::Less, Ordering::Equal, Ordering::Greater] {
        for expected_y in [Ordering::Less, Ordering::Equal, Ordering::Greater] {
            let mut mem = Memoization3::new(
                |mem, pos: usize, cur_x: Ordering, cur_y: Ordering| -> Option<(i64, i64, i64)> {
                    if pos == 32 {
                        if cur_x == expected_x && cur_y == expected_y {
                            Some((0, 0, 0))
                        } else {
                            None
                        }
                    } else {
                        let mut res = None;
                        for (x_bit, y_bit) in [(true, false), (false, true), (false, false)] {
                            let nx = if x_bit == x.is_set(pos) {
                                cur_x
                            } else if x_bit {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            };
                            let ny = if y_bit == y.is_set(pos) {
                                cur_y
                            } else if y_bit {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            };
                            if let Some((mut val, mut p, mut q)) = mem.call(pos + 1, nx, ny) {
                                if expected_x == Ordering::Greater {
                                    if x_bit {
                                        val += 1 << pos;
                                    }
                                    if x.is_set(pos) {
                                        val -= 1 << pos;
                                    }
                                } else if expected_x == Ordering::Less {
                                    if x_bit {
                                        val -= 1 << pos;
                                    }
                                    if x.is_set(pos) {
                                        val += 1 << pos;
                                    }
                                }
                                if expected_y == Ordering::Greater {
                                    if y_bit {
                                        val += 1 << pos;
                                    }
                                    if y.is_set(pos) {
                                        val -= 1 << pos;
                                    }
                                } else if expected_y == Ordering::Less {
                                    if y_bit {
                                        val -= 1 << pos;
                                    }
                                    if y.is_set(pos) {
                                        val += 1 << pos;
                                    }
                                }
                                p *= 2;
                                if x_bit {
                                    p += 1;
                                }
                                q *= 2;
                                if y_bit {
                                    q += 1;
                                }
                                res.minim((val, p, q));
                            }
                        }
                        res
                    }
                },
            );
            if let Some(cand) = mem.call(0, Ordering::Equal, Ordering::Equal) {
                ans.minim(cand);
            }
        }
    }
    let (_, p, q) = ans.unwrap();
    out.print_line((p, q));
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
        }
    }
    eprint!("\x1B[0m");
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
