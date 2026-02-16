//{"name":"H. Corridor","group":"Codeforces - Constructor Open Cup 2026","url":"https://constructor2026.contest.codeforces.com/group/XdjJUfzFUt/contest/670933/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n11\n10\n2\n00\n11\n3\n000\n000\n1\n1\n1\n","output":"YES\n+-\n--\nNO\nYES\n-+-\n++-\nYES\n+\n+\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_char_table(2, n);

    let mut ans = Arr2d::new(2, n, b'-');
    if s[(0, 0)] == b'1' {
        ans[(0, 0)] = b'+';
    }
    for i in 1..n {
        let need_up = s[(0, i)] == b'1';
        let need_down = s[(1, i - 1)] == b'1';
        let can_up = s[(0, i - 1)] == b'1';
        let can_down = if i == 1 { false } else { s[(1, i - 2)] == b'1' };
        if need_up && !can_up {
            out.print_line(false);
            return;
        }
        if need_down && !can_up && !can_down {
            out.print_line(false);
            return;
        }
        if can_up && i % 2 == 1 && (!need_up || !need_down) {
            out.print_line(false);
            return;
        }
        if can_down && i % 2 == 1 && !need_down {
            out.print_line(false);
            return;
        }
        if i % 2 == 0 {
            if need_up {
                ans[(0, i)] = b'+';
            }
            if need_down {
                ans[(1, i - 1)] = b'+';
            }
        }
    }
    if s[(1, n - 1)] == b'1' {
        if s[(0, n - 1)] != b'1' && (n == 1 || s[(1, n - 2)] != b'1') {
            out.print_line(false);
            return;
        }
        ans[(1, n - 1)] = b'+';
    } else if n % 2 == 1 {
        if s[(0, n - 1)] == b'1' || n != 1 && s[(1, n - 2)] == b'1' {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
    out.print_table(&ans);
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
