//{"name":"P2 - Heavy-Light Composition","group":"DMOJ - AAAA 1","url":"https://dmoj.ca/problem/aaaa1p2","interactive":false,"timeLimit":1000,"tests":[{"input":"1 3\n5\n3 4 7\n","output":"4\n"},{"input":"2 2\n2 7\n1 5\n","output":"3\n"},{"input":"2 4\n1 10\n2 5 7 8\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let l = input.read_int_vec(n);
    let f = input.read_int_vec(m);

    let mut all = Vec::new();
    for i in l.copy_iter() {
        all.push((i, true));
    }
    for i in f.copy_iter() {
        all.push((i, false));
    }
    all.sort();
    let mut ans = 0;
    let mut last = all[0].0;
    let mut max = 0;
    let mut enabled = false;
    for (pos, light) in all {
        let cur = pos - last;
        ans += cur;
        if enabled {
            max.maxim(cur);
        }
        if light {
            ans -= max;
            max = 0;
            enabled = true;
        }
        last = pos;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
