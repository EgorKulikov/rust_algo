//{"name":"C - Coverage Area of Radio Towers","group":"AtCoder - AtCoder Weekday Contest 0053 Beta","url":"https://atcoder.jp/contests/awc0053/tasks/awc0053_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 1 2 5\n5 2 0 4\n6 1 1 3\n","output":"9\n"},{"input":"4\n0 0 0 7\n0 1 2 3\n3 1 0 5\n10 0 0 1\n","output":"10\n"},{"input":"8\n1 1 0 2\n4 2 3 5\n6 0 2 4\n8 3 1 6\n10 5 0 3\n10 0 4 7\n13 2 2 1\n20 10 0 8\n","output":"18\n"},{"input":"15\n0 0 5 2\n2 1 2 4\n4 3 0 6\n7 2 5 3\n9 0 0 8\n12 4 1 5\n15 5 5 7\n18 3 2 4\n18 0 6 9\n23 10 0 1\n25 2 3 6\n30 8 4 5\n35 0 0 10\n40 7 7 2\n50 20 0 8\n","output":"21\n"},{"input":"1\n1000000000 1000000000 1000000000 10000\n","output":"10000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut events = Vec::new();
    for _ in 0..n {
        let x = input.read_long();
        let l = input.read_long();
        let r = input.read_long();
        let c = input.read_long();
        events.push((x - l, c));
        events.push((x + r, -c));
    }
    events.sort_by_key(|&(p, c)| (p, Reverse(c)));
    let mut cur = 0;
    let mut ans = 0;
    for (_, c) in events {
        cur += c;
        ans.maxim(cur);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
