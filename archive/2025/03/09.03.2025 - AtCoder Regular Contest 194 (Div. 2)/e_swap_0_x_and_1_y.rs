//{"name":"E - Swap 0^X and 1^Y","group":"AtCoder - AtCoder Regular Contest 194 (Div. 2)","url":"https://atcoder.jp/contests/arc194/tasks/arc194_e","interactive":false,"timeLimit":2000,"tests":[{"input":"9 2 1\n000111001\n011000011\n","output":"Yes\n"},{"input":"1 1 1\n0\n1\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let x = input.read_size();
    let y = input.read_size();
    let s = input.read_str();
    let t = input.read_str();

    out.set_bool_output(BoolOutput::YesNo);
    if s.copy_count(b'0') != t.copy_count(b'0') {
        out.print_line(false);
        return;
    }
    let mut s = s
        .iter_enumerate()
        .filter(|&(_, c)| c == b'1')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut t = t
        .iter_enumerate()
        .filter(|&(_, c)| c == b'1')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    for i in s.indices() {
        if s[i] == t[i] {
            continue;
        }
        if s[i] < t[i] {
            swap(&mut s, &mut t);
        }
        let delta = s[i] - t[i];
        if delta % x != 0 {
            out.print_line(false);
            return;
        }
        let mut target = Vec::new();
        for j in 0..y {
            target.push(t[i] + j);
        }
        let mut was_s = s[i];
        s[i] = t[i];
        for j in 1.. {
            if j == target.len() {
                break;
            }
            if i + j == s.len() {
                out.print_line(false);
                return;
            }
            if s[i + j] != was_s + 1 {
                if (s[i + j] - (was_s + 1)) % x != 0 {
                    out.print_line(false);
                    return;
                }
                while j + y > target.len() {
                    target.push(was_s + 1 + target.len() - j);
                }
            }
            was_s = s[i + j];
            s[i + j] = target[j];
        }
    }
    out.print_line(true);
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
