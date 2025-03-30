//{"name":"J4 - Sunny Days","group":"DMOJ - CCC '25","url":"https://dmoj.ca/problem/ccc25j4","interactive":false,"timeLimit":1000,"tests":[{"input":"8\nP\nS\nP\nS\nS\nP\nP\nS\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_vec::<u8>(n);

    let mut prev = 0;
    let mut cur = 0;
    let mut ans = 0;
    let mut wet = 0;
    for c in s.copy_iter() {
        match c {
            b'P' => {
                if wet == 0 {
                    ans.maxim(cur + 1);
                    prev = cur + 1;
                    cur = 0;
                }
                wet += 1;
                if wet > 1 {
                    prev = 1;
                }
            }
            b'S' => {
                wet = 0;
                cur += 1;
                ans.maxim(cur + prev);
            }
            _ => unreachable!(),
        }
    }
    if ans == n && s.copy_count(b'S') == n {
        ans -= 1;
    }
    out.print_line(ans);
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
