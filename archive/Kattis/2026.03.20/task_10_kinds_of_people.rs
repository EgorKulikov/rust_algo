//{"name":"10 Kinds of People","group":"Kattis","url":"https://open.kattis.com/problems/tenkindsofpeople","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n11 10\n1Z 123\n10 4000\n1111 3333\n","output":"3 2 3\n123 62 10\n500 500 5\nCANNOT MAKE EQUAL\n"},{"input":"4\n75 4GH\nteH9x eR8LJ\n231Tq jIhE7\nNVWng Oiing\n","output":"11275 1610 48\n490386017 64 76\nCANNOT MAKE EQUAL\nCANNOT MAKE EQUAL\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_str();
    let b = input.read_str();

    fn digit(c: u8) -> i64 {
        if c.is_ascii_digit() {
            (c - b'0') as i64
        } else if c.is_ascii_lowercase() {
            (c - b'a' + 10) as i64
        } else {
            (c - b'A' + 36) as i64
        }
    }

    fn all(a: &Str) -> FxHashMap<i64, i64> {
        let mut res = FxHashMap::default();
        for base in 2..=7500 {
            let mut cur = 0;
            for c in a.copy_iter() {
                let val = digit(c);
                if val >= base {
                    cur = -1;
                    break;
                }
                cur *= base;
                cur += val;
            }
            if cur != -1 {
                res.insert(cur, base);
            }
        }
        res
    }
    let a = all(&a);
    let b = all(&b);
    for (a, val) in a {
        if let Some(b) = b.get(&a) {
            out.print_line((a, val, *b));
            return;
        }
    }
    out.print_line("CANNOT MAKE EQUAL");
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
