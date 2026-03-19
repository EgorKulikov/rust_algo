//{"name":"E - Making Rope from Materials","group":"AtCoder - AtCoder Weekday Contest 0024 Beta","url":"https://atcoder.jp/contests/awc0024/tasks/awc0024_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 10\n3 2\n5 3\n7 1\n","output":"2\n"},{"input":"2 7\n3 2\n5 1\n","output":"-1\n"},{"input":"5 80\n7 5\n13 4\n23 3\n31 2\n50 2\n","output":"3\n"},{"input":"10 5000\n3 100\n7 200\n11 150\n19 80\n29 60\n53 40\n97 30\n181 20\n337 15\n631 8\n","output":"12\n"},{"input":"1 1\n1 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let w = input.read_size();
    let lc = input.read_size_pair_vec(n);

    let mut ans = vec![None; w + 1];
    ans[0] = Some(0);
    let mut next = vec![None; w + 1];
    for (l, c) in lc {
        next.fill(None);
        for base in 0..l {
            let mut set = BTreeSet::new();
            for i in 0..=(w - base) / l {
                let at = base + i * l;
                if let Some(val) = ans[at] {
                    set.insert((val - i as i32, i));
                }
                if let Some(&(val, _)) = set.first() {
                    next[at].minim(val + i as i32);
                }
                if i >= c {
                    if let Some(val) = ans[at - c * l] {
                        set.remove(&(val - (i - c) as i32, i - c));
                    }
                }
            }
        }
        swap(&mut ans, &mut next);
    }
    out.print_line(ans[w]);
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
