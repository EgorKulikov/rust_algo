//{"name":"A. Automatized Mineral Classification","group":"Universal Cup - CERC 2025","url":"https://contest.ucup.ac/contest/2814/problem/16000","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2 1\n","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, Shuffle};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut r = Random::new_with_seed(239);
    let mut order = (0..n).collect::<Vec<_>>();
    order.shuffle_with(&mut r);

    let mut classes: Vec<Vec<usize>> = Vec::new();
    let mut belt = VecDeque::new();
    let mut special = Vec::new();
    let mut select = vec![Vec::new(); n];
    for i in order {
        writeln!(out, "+ {}", i + 1).unwrap();
        out.flush();
        let num = input.read_size();
        if num > classes.len() {
            classes.push(vec![i]);
            belt.push_back(classes.len() - 1);
            continue;
        } else {
            special.push(i);
            belt.push_back(usize::MAX);
            if special.len() == 20 {
                let mut expected = classes.len();
                let mut cand = Vec::new();
                while !belt.is_empty() {
                    out.print_line('-');
                    out.flush();
                    let cur = belt.pop_front().unwrap();
                    let num = input.read_size();
                    if num != expected - 1 {
                        if cur != usize::MAX {
                            cand.push(cur);
                        }
                    }
                    expected = num;
                }
                for i in special.copy_iter() {
                    select[i] = cand.clone();
                }
                special.clear();
                for i in classes.indices() {
                    writeln!(out, "+ {}", classes[i][0] + 1).unwrap();
                    belt.push_back(i);
                    out.flush();
                    input.read_size();
                }
            }
        }
    }
    let mut expected = classes.len();
    let mut cand = Vec::new();
    while !belt.is_empty() {
        out.print_line('-');
        out.flush();
        let cur = belt.pop_front().unwrap();
        let num = input.read_size();
        if num != expected - 1 {
            if cur != usize::MAX {
                cand.push(cur);
            }
        }
        expected = num;
    }
    for i in special.copy_iter() {
        select[i] = cand.clone();
    }
    for k in 0..n {
        if select[k].is_empty() {
            continue;
        }
        writeln!(out, "+ {}", k + 1).unwrap();
        out.flush();
        input.read_size();
        let mut expected = 1;
        for c in select[k].copy_iter() {
            writeln!(out, "+ {}", classes[c][0] + 1).unwrap();
            out.flush();
            let num = input.read_size();
            if num != expected + 1 {
                classes[c].push(k);
                for _ in 0..=expected {
                    writeln!(out, "-").unwrap();
                    out.flush();
                    input.read_size();
                }
                break;
            }
            expected += 1;
        }
    }
    writeln!(out, "! {}", classes.len()).unwrap();
    for c in classes {
        out.print_line((c.len(), c.inc()));
    }
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
