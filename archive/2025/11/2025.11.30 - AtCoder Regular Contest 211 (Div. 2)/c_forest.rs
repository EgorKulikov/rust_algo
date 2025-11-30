//{"name":"C - Forest","group":"AtCoder - AtCoder Regular Contest 211 (Div. 2)","url":"https://atcoder.jp/contests/arc211/tasks/arc211_c","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n...###..\n20 25 1 1 30 2 1 1\n","output":"2\n"},{"input":"5\n.#.#.\n211 182 192 182 211\n","output":"2\n"},{"input":"11\n#..#.##.#..\n192 192 192 211 182 192 182 192 182 211 182\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let r = input.read_int_vec(n);

    let mut cur = 0;
    let mut qty = 0usize;
    let mut groups = Vec::new();
    let mut fg = Vec::new();
    let mut f_cur = 0;
    for i in 0..n {
        if s[i] == b'#' {
            if cur != 0 {
                groups.push((cur, qty));
            }
            cur = 0;
            qty = 0;
            f_cur.maxim(r[i]);
        } else {
            if cur.maxim(r[i]) {
                qty = 1;
            } else if cur == r[i] {
                qty += 1;
            }
            if f_cur != 0 && !groups.is_empty() {
                fg.push(f_cur);
            }
            f_cur = 0;
        }
    }
    if cur != 0 {
        groups.push((cur, qty));
    }
    let mut ans = 0;
    let max = groups.copy_max().0.max(fg.copy_max());
    for (i, ((v0, q0), (v1, q1))) in groups.consecutive_iter_copy().enumerate() {
        if v0 == max || v1 == max || fg[i] == max {
            ans += q0 * q1;
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
