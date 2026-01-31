//{"name":"T727428 [语言月赛 202601] 洛谷网校","group":"Luogu","url":"https://www.luogu.com.cn/problem/T727428?contestId=304401","interactive":false,"timeLimit":1000,"tests":[{"input":"4 20\n4 1 3 5 7 9 11 16 18\n4 2 4 6 7 7 9 11 16\n4 4 5 7 8 10 11 17 18\n4 2 4 6 8 13 15 18 19\n","output":"15\n"},{"input":"4 15\n2 1 2 10 12\n1 11 14\n1 15 15\n1 15 15\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::option::OptionExt;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut st = Vec::with_capacity(n);
    for _ in 0..n {
        let c = input.read_size();
        st.push(input.read_size_pair_vec(c));
    }

    let mut at = 0;
    for i in 0..n {
        let mut best = m + 1;
        for (s, t) in st[i].copy_iter() {
            if s > at {
                best.minim(t);
            }
        }
        at = best;
    }
    out.print_line(at.take_if(at <= m));
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
