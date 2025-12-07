//{"name":"B. Открытки с пожеланиями","group":"Codeforces - Codeforces Round 1069 (Div. 1)","url":"https://codeforces.com/contest/2174/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 4\n0 0 1\n6 8\n1 2 0 5 1 8\n3 4\n1 0 4\n5 8\n2 4 5 4 3\n","output":"1\n20\n5\n19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_4d::Memoization4d;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let mut b = Vec::new();
    let mut last = 0;
    let mut qty = 0;
    for x in a {
        if x > last {
            if last != 0 {
                b.push((last, qty));
            }
            last = x;
            qty = 0;
        }
        qty += 1;
    }
    if last != 0 {
        b.push((last, qty));
    }
    let mut mem = Memoization4d::new(
        b.len() + 1,
        k + 1,
        k + 1,
        2,
        |mem, pos, cur, rem, taken| -> usize {
            if pos == b.len() {
                0
            } else {
                let mut res = mem.call(pos + 1, cur, rem, 0) + cur * b[pos].1;
                if taken == 1 && rem > 0 && cur < b[pos].0 {
                    res.maxim(mem.call(pos, cur + 1, rem - 1, 1));
                } else if taken == 0 && rem > cur {
                    res.maxim(mem.call(pos, cur + 1, rem - cur - 1, 1));
                }
                res
            }
        },
    );
    out.print_line(mem.call(0, 0, k, 0));
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
