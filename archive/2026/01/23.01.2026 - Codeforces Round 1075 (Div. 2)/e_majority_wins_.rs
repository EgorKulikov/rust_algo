//{"name":"E. Majority Wins?","group":"Codeforces - Codeforces Round 1075 (Div. 2)","url":"https://codeforces.com/contest/2189/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n0\n1\n1\n2\n00\n2\n10\n3\n010\n8\n00111000\n6\n100100\n","output":"-1\n0\n-1\n2\n4\n9\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::string_search::StringSearch;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let ones = s.copy_count(b'1');
    let zeros = n - ones;

    let mut bal = 0;
    let mut more = false;
    let mut same = false;
    for i in 0..n {
        if s[i] == b'1' {
            bal += 1;
        } else {
            bal -= 1;
        }
        if bal > 0 {
            more = true;
            break;
        }
        if bal == 0 {
            same = true;
        }
    }
    bal = 0;
    for i in (0..n).rev() {
        if s[i] == b'1' {
            bal += 1;
        } else {
            bal -= 1;
        }
        if bal > 0 {
            more = true;
            break;
        }
        if bal == 0 {
            same = true;
        }
    }

    if ones == 0 {
        out.print_line(-1);
    } else if n == 1 {
        out.print_line(0);
    } else if ones >= zeros {
        out.print_line(n);
    } else if s[0] == b'1' || s[Back(0)] == b'1' || ones + 1 == zeros || more {
        out.print_line(n + 1);
    } else if same || s.str_contains(b"11") {
        out.print_line(n + 2);
    } else {
        out.print_line(n + 3);
    }
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
