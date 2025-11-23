//{"name":"G. Isaac's Queries","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/G","interactive":true,"timeLimit":4000,"tests":[{"input":"1\n3\n\n2\n\n-1\n\n1\n","output":"\n\n? 1 2\n\n? 1 3\n\n? 2 3\n\n!\n1 2 -1\n2 1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    if n == 3 {
        let mut ans = Vec::new();
        for i in 0..3 {
            let mut cur = Vec::new();
            for j in i..3 {
                writeln!(out, "? {} {}", i + 1, j + 1).unwrap();
                out.flush();
                let res = input.read_int();
                if res == -2 {
                    return;
                }
                cur.push(res);
            }
            ans.push(cur);
        }
        out.print_line(b'!');
        out.print_per_line(&ans);
        return;
    }
    assert_eq!(n, 100);
    let mut ans = Arr2d::new(n + 1, n + 1, None);
    let mut update = |l: usize, r: usize| {
        if ans[(l, r)].is_some() {
            return;
        }
        writeln!(out, "? {} {}", l + 1, r).unwrap();
        out.flush();
        let res = input.read_int();
        if res == -2 {
            panic!();
        }
        let mut queue = vec![(l, r, res)];
        ans[(l, r)] = Some(res);
        ans[(r, l)] = Some(res);
        while let Some((l, r, res)) = queue.pop() {
            for j in 0..n {
                if let Some(r2) = ans[(j, l)] {
                    if r2 != res && ans[(j, r)].is_none() {
                        ans[(j, r)] = Some(res.max(r2));
                        ans[(r, j)] = Some(res.max(r2));
                        queue.push((j, r, res.max(r2)));
                    }
                }
                if let Some(r2) = ans[(j, r)] {
                    if r2 != res && ans[(j, l)].is_none() {
                        ans[(j, l)] = Some(res.max(r2));
                        ans[(l, j)] = Some(res.max(r2));
                        queue.push((j, l, res.max(r2)));
                    }
                }
            }
        }
    };
    for len in (1..=n).rev() {
        for l in 0..=n - len {
            update(l, l + len);
        }
    }
    out.print_line(b'!');
    for i in 0..n {
        out.print_line_iter(ans.row(i).skip(i + 1));
    }
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
