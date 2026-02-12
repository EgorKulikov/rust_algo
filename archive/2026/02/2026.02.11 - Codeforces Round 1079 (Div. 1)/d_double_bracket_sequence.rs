//{"name":"D. Double Bracket Sequence","group":"Codeforces - Codeforces Round 1079 (Div. 1)","url":"https://codeforces.com/contest/2196/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n[)\n4\n[)[(\n4\n))[[\n4\n([)]\n6\n[)]](]\n","output":"1\n2\n2\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::{Callable2, Callable3, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut bad_close = Vec::new();
    let mut bad_close_square = Vec::new();
    let mut bal = 0;
    let mut bal_square = 0;
    for i in 0..n {
        match s[i] {
            b'(' => {
                bal += 1;
            }
            b')' => {
                if bal == 0 {
                    bad_close.push(i);
                } else {
                    bal -= 1;
                }
            }
            b'[' => {
                bal_square += 1;
            }
            b']' => {
                if bal_square == 0 {
                    bad_close_square.push(i);
                } else {
                    bal_square -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
    let mut bad_open = Vec::new();
    let mut bad_open_square = Vec::new();
    let mut bal = 0;
    let mut bal_square = 0;
    for i in (0..n).rev() {
        match s[i] {
            b'(' => {
                if bal == 0 {
                    bad_open.push(i);
                } else {
                    bal -= 1;
                }
            }
            b')' => {
                bal += 1;
            }
            b'[' => {
                if bal_square == 0 {
                    bad_open_square.push(i);
                } else {
                    bal_square -= 1;
                }
            }
            b']' => {
                bal_square += 1;
            }
            _ => unreachable!(),
        }
    }
    let bad = [bad_open, bad_close, bad_open_square, bad_close_square];
    let mut rec = RecursiveFunction2::new(|rec, step: usize, mut pos: Vec<usize>| -> usize {
        if step == 4 {
            pos.sort();
            let mut mem = Memoization3d::new(
                pos.len() + 1,
                pos.len() / 2 + 1,
                pos.len() / 2 + 1,
                |mem, p, op, sq_op| -> usize {
                    if p == pos.len() {
                        if op == 0 && sq_op == 0 {
                            0
                        } else {
                            usize::MAX / 2
                        }
                    } else {
                        let mut res = usize::MAX / 2;
                        if op > 0 {
                            res.minim(
                                mem.call(p + 1, op - 1, sq_op)
                                    + if s[pos[p]] == b')' { 0 } else { 1 },
                            );
                        }
                        if op < pos.len() / 2 {
                            res.minim(
                                mem.call(p + 1, op + 1, sq_op)
                                    + if s[pos[p]] == b'(' { 0 } else { 1 },
                            );
                        }
                        if sq_op > 0 {
                            res.minim(
                                mem.call(p + 1, op, sq_op - 1)
                                    + if s[pos[p]] == b']' { 0 } else { 1 },
                            );
                        }
                        if sq_op < pos.len() / 2 {
                            res.minim(
                                mem.call(p + 1, op, sq_op + 1)
                                    + if s[pos[p]] == b'[' { 0 } else { 1 },
                            );
                        }
                        res
                    }
                },
            );
            return mem.call(0, 0, 0);
        }
        if bad[step].is_empty() {
            // if bad[step].len() % 2 == 0 {
            rec.call(step + 1, pos) + bad[step].len() / 2
        } else if bad[step].len() % 2 == 1 {
            let add = bad[step].len() / 2;
            let mut res = rec.call(step + 1, pos.clone().do_with(|x| x.push(bad[step][0])));
            if bad[step].len() > 1 {
                res.minim(rec.call(
                    step + 1,
                    pos.clone().do_with(|x| x.push(bad[step][Back(0)])),
                ));
            }
            res + add
        } else {
            let add = bad[step].len() / 2 - 1;
            let mut res = rec.call(
                step + 1,
                pos.clone().do_with(|x| {
                    x.push(bad[step][0]);
                    x.push(bad[step][1]);
                }),
            );
            if bad[step].len() > 2 {
                res.minim(rec.call(
                    step + 1,
                    pos.clone().do_with(|x| {
                        x.push(bad[step][0]);
                        x.push(bad[step][Back(0)]);
                    }),
                ));
                res.minim(rec.call(
                    step + 1,
                    pos.clone().do_with(|x| {
                        x.push(bad[step][Back(1)]);
                        x.push(bad[step][Back(0)]);
                    }),
                ));
            }
            res + add
        }
    });
    out.print_line(rec.call(0, Vec::new()));
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
