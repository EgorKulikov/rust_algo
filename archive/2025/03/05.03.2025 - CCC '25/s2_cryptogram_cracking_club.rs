//{"name":"S2 - Cryptogram Cracking Club","group":"DMOJ - CCC '25","url":"https://dmoj.ca/problem/ccc25s2","interactive":false,"timeLimit":8000,"tests":[{"input":"r2d2\n8\n","output":"r\n"},{"input":"a4b1c2d10\n100\n","output":"d\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let c = input.read_long();

    let mut rle = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for c in s {
        if c.is_ascii_alphabetic() {
            if y != 0 {
                rle.push((x, y));
                y = 0;
            }
            x = c;
        } else {
            y = y * 10 + c as i64 - b'0' as i64;
        }
    }
    rle.push((x, y));
    let len = rle.copy_fold(0, |acc, (_, y)| acc + y);
    let mut c = c % len;
    for (x, y) in rle {
        if c < y {
            out.print_line(x);
            return;
        }
        c -= y;
    }
    unreachable!();
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

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
        TaskType::Classic => input.is_empty(),
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
