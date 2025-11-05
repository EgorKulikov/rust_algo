//{"name":"B. Locate","group":"Codeforces - Testing Round 20 (Unrated, Communication Problems)","url":"https://codeforces.com/contest/2168/problem/B","interactive":false,"timeLimit":3000,"tests":[{"input":"first\n3\n3\n3 2 1\n5\n1 2 3 4 5\n5\n4 2 3 5 1\n","output":"0\n0\n1\n"},{"input":"second\n3\n3 0\n\n2\n\n1\n\n1\n\n5 1\n\n2\n\n5 0\n\n4\n\n0\n","output":"\n\n\n? 1 3\n\n? 1 2\n\n? 2 3\n\n! 1\n\n? 1 2\n\n! 4\n\n? 1 5\n\n? 5 5\n\n! 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mode = input.read_str();

    if mode.as_slice() == b"first" {
        let t = input.read_int();
        for _ in 0..t {
            let n = input.read_size();
            let p = input.read_size_vec(n).dec();
            let q = p.inv();
            out.print_line((q[0] > q[n - 1]) as usize);
            out.flush();
        }
    } else {
        let t = input.read_int();
        for _ in 0..t {
            let n = input.read_size();
            let x = input.read_size();
            let mut left = 1;
            let mut right = n - 1;
            while left < right {
                let mid = (left + right) / 2;
                writeln!(out, "? 1 {}", mid + 1).unwrap();
                out.flush();
                if input.read_size() == n - 1 {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            let last = left;
            left = 0;
            right = last - 1;
            while left < right {
                let mid = (left + right + 1) / 2;
                writeln!(out, "? {} {}", mid + 1, last + 1).unwrap();
                out.flush();
                if input.read_size() == n - 1 {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            if x == 0 {
                writeln!(out, "! {}", last + 1).unwrap();
            } else {
                writeln!(out, "! {}", left + 1).unwrap();
            }
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
    match TASK_TYPE {
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
        TaskType::Interactive => true,
    }
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
