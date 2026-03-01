//{"name":"F. Simons and Reconstructing His Roads","group":"Codeforces - Codeforces Round 1083 (Div. 2)","url":"https://codeforces.com/contest/2205/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 4\n2 3 -2 3\n4 9 4 -4\n3 4 -2\n-9 -5 1\n6 -1 -3\n1111\n1111\n111\n111\n111\n2 4\n4 23 1 35\n6 12 -17\n-14 1 -40\n0100\n000\n101\n3 3\n1 0 1\n0 1 0\n1 0\n0 1\n0 0\n110\n111\n10\n11\n11\n3 4\n13 7 6 -12\n3 -5 12 -6\n-3 10 -15\n-5 8 -11\n10 0 -5\n1111\n0110\n110\n101\n010\n","output":"38\n0\n4\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu2d::DSU2d;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let w = input.read_long_table(n - 1, m);
    let v = input.read_long_table(n, m - 1);
    let p = input.read_char_table(n - 1, m);
    let q = input.read_char_table(n, m - 1);

    let mut val = Arr2d::with_gen(n - 1, m - 1, |i, j| {
        w[(i, j)] - w[(i, j + 1)] + v[(i, j)] - v[(i + 1, j)]
    });
    let mut dsu = DSU2d::new(n - 1, m - 1);
    let mut bad = Arr2d::new(n - 1, m - 1, false);
    for i in 0..n - 1 {
        if p[(i, 0)] == b'0' {
            bad[dsu.find(i, 0)] = true;
        }
        if p[(i, m - 1)] == b'0' {
            bad[dsu.find(i, m - 2)] = true;
        }
        for j in 1..m - 1 {
            if p[(i, j)] == b'0' {
                let a = dsu.find(i, j - 1);
                let b = dsu.find(i, j);
                if a != b {
                    bad[a] |= bad[b];
                    val[a] += val[b];
                    dsu.union(a.0, a.1, b.0, b.1);
                }
            }
        }
    }
    for j in 0..m - 1 {
        if q[(0, j)] == b'0' {
            bad[dsu.find(0, j)] = true;
        }
        if q[(n - 1, j)] == b'0' {
            bad[dsu.find(n - 2, j)] = true;
        }
        for i in 1..n - 1 {
            if q[(i, j)] == b'0' {
                let a = dsu.find(i - 1, j);
                let b = dsu.find(i, j);
                if a != b {
                    bad[a] |= bad[b];
                    val[a] += val[b];
                    dsu.union(a.0, a.1, b.0, b.1);
                }
            }
        }
    }
    let mut ans = 0;
    for (i, j) in val.indices() {
        if dsu.find(i, j) == (i, j) && !bad[(i, j)] && val[(i, j)] > 0 {
            ans += val[(i, j)];
        }
    }
    out.print_line(ans);
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
