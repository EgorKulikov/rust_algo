//{"name":"All about that base","group":"Kattis","url":"https://open.kattis.com/problems/allaboutthatbase","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n6ef + d1 = 7c0\n3 / 2 = 1\n444 / 2 = 222\n10111 * 11 = 1000101\n10111 * 11 = 111221\n5k - 1z = 46\n1111111111 - 1111111 = 111\n2048 - 512 = 1536\n","output":"g\ninvalid\n56789abcdefghijklmnopqrstuvwxyz0\n2\n3456789abcdefghijklmnopqrstuvwxyz0\ninvalid\n1\na\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::option::OptionExt;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_str();
    let op = input.read_char();
    let b = input.read_str();
    assert_eq!(input.read_char(), b'=');
    let c = input.read_str();

    fn convert(s: &Str, base: i64) -> Option<i64> {
        if base == 1 {
            if s.copy_count(b'1') == s.len() {
                return Some(s.len() as i64);
            } else {
                return None;
            }
        }
        let mut res = 0i64;
        for c in s.copy_iter() {
            let val = if c.is_ascii_digit() {
                c - b'0'
            } else {
                c - b'a' + 10
            } as i64;
            if val >= base {
                return None;
            }
            res = res.checked_mul(base)?;
            res = res.checked_add(val)?;
        }
        res.take_if(res >= 1 && res <= (1 << 32) - 1)
    }
    let mut ans = Str::new();
    for i in 1..=36 {
        let va = if let Some(va) = convert(&a, i) {
            va
        } else {
            continue;
        };
        let vb = if let Some(vb) = convert(&b, i) {
            vb
        } else {
            continue;
        };
        let vc = if let Some(vc) = convert(&c, i) {
            vc
        } else {
            continue;
        };
        let valid = match op {
            b'+' => va + vb == vc,
            b'-' => va - vb == vc,
            b'*' => va * vb == vc,
            b'/' => vb * vc == va,
            _ => unreachable!(),
        };
        if valid {
            if i < 10 {
                ans.push(b'0' + i as u8);
            } else if i < 36 {
                ans.push(b'a' + (i as u8 - 10));
            } else {
                ans.push(b'0');
            }
        }
    }
    if ans.is_empty() {
        out.print_line("invalid");
    } else {
        out.print_line(ans);
    }
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
