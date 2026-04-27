//{"name":"E. Seek the Truth","group":"Codeforces - Spectral::Cup 2026 Round 1 (Codeforces Round 1094, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2222/problem/E","interactive":true,"timeLimit":4000,"tests":[{"input":"3\n2\n\n\n1\n\n2\n\n\n2\n\n1\n\n2\n\n2\n\n3\n\n2\n\n\n2\n\n2\n\n2\n","output":"\n\n3\nI 3\n\nA 1 3\n\n1\nI 0\n\nQ 2\n\nQ 1\n\nQ 0\n\nI 3\n\nA 2 2\n\n1\nI 1\n\nI 0\n\nA 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    out.print_line(0);
    out.flush();
    writeln!(out, "I {}", 0).unwrap();
    out.flush();

    if input.read_size() == 2 {
        let mut left = 1;
        let mut right = (1i64 << n) - 1;
        while left < right {
            let mid = (left + right + 1) / 2;
            writeln!(out, "Q {}", mid).unwrap();
            out.flush();
            if input.read_size() == 1 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        let c = left;
        if (c & (c - 1)) == 0 {
            writeln!(out, "I {}", i64::all_bits(n)).unwrap();
            out.flush();
            input.read_size();
            writeln!(out, "Q {}", i64::all_bits(n)).unwrap();
            out.flush();
            if input.read_size() == 1 {
                writeln!(out, "A 2 {}", c).unwrap();
            } else {
                writeln!(out, "A 3 {}", c).unwrap();
            }
            out.flush();
            return;
        } else {
            writeln!(out, "I {}", 1i64 << c.highest_bit()).unwrap();
            out.flush();
            if input.read_size() == 3 {
                writeln!(out, "A 3 {}", c).unwrap();
            } else {
                writeln!(out, "A 2 {}", c).unwrap();
            }
            out.flush();
            return;
        }
    } else {
        let mut size = 1;
        let mut ans = 0i64;
        for i in 0..n {
            writeln!(out, "I {}", 1i64 << i).unwrap();
            out.flush();
            if input.read_size() == size + 1 {
                size += 1;
                ans.set_bit(i);
            }
        }
        writeln!(out, "A 1 {}", ans).unwrap();
        out.flush();
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
