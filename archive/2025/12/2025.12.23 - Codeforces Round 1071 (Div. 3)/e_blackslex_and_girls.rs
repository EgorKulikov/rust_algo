//{"name":"E. Blackslex and Girls","group":"Codeforces - Codeforces Round 1071 (Div. 3)","url":"https://codeforces.com/contest/2179/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 5 5\n010\n2 4 3\n4 2 3\n0001\n1 1 1 1\n2 4 2\n00\n3 3\n4 23 20\n1111\n2 2 2 2\n1 25 26\n0\n51\n2 4 2\n00\n3 4\n","output":"YES\nNO\nYES\nNO\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let y = input.read_long();
    let s = input.read_str();
    let p = input.read_long_vec(n);

    let total = p.copy_sum();
    if total > x + y {
        out.print_line(false);
        return;
    }
    for (qty, target) in [(x, b'0'), (y, b'1')] {
        let mut need = 0;
        for i in 0..n {
            if s[i] == target {
                need += p[i] / 2 + 1;
            }
        }
        if need > qty {
            out.print_line(false);
            return;
        }
    }
    if s.copy_all(|c| c == b'0') && x < y + n as i64 {
        out.print_line(false);
        return;
    }
    if s.copy_all(|c| c == b'1') && y < x + n as i64 {
        out.print_line(false);
        return;
    }
    out.print_line(true);
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
