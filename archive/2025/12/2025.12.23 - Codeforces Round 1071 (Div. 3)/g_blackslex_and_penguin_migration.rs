//{"name":"G. Blackslex and Penguin Migration","group":"Codeforces - Codeforces Round 1071 (Div. 3)","url":"https://codeforces.com/contest/2179/problem/G","interactive":true,"timeLimit":5000,"tests":[{"input":"2\n\n1\n\n2\n\n1\n\n1\n\n2\n\n1\n\n3\n\n3\n","output":"? 1 2\n\n? 1 3\n\n? 1 4\n\n? 2 3\n\n? 2 4\n\n? 3 4\n\n!\n3 4\n2 1\n\n? 1 8\n\n!\n9 1 3\n4 2 7\n8 5 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ask = |a: usize, b: usize| -> usize {
        use std::io::Write;
        writeln!(out, "? {} {}", a + 1, b + 1).unwrap();
        out.flush();
        input.read_size()
    };

    let d0 = Vec::with_gen(n * n, |i| ask(0, i));
    let top_left = d0.max_position();
    let dtl = Vec::with_gen(n * n, |i| ask(top_left, i));
    let mut top_right = n * n;
    let mut dst = None;
    let mut from = 0;
    while dtl[from] == 0 || dtl[from] == 2 * (n - 1) {
        from += 1;
    }
    for i in 0..n * n {
        if dtl[i] == n - 1 && dst.maxim(ask(from, i)) {
            top_right = i;
        }
    }
    let dtr = Vec::with_gen(n * n, |i| ask(top_right, i));
    let mut ans = Arr2d::new(n, n, 0);
    for i in 0..n * n {
        let r = (dtl[i] + dtr[i] - (n - 1)) / 2;
        let c = dtl[i] - r;
        assert_eq!(ans[(r, c)], 0);
        ans[(r, c)] = i + 1;
    }
    out.print_line("!");
    out.print_line(ans);
    out.flush();
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
