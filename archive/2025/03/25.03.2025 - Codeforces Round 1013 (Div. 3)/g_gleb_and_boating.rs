//{"name":"G. Gleb and Boating","group":"Codeforces - Codeforces Round 1013 (Div. 3)","url":"https://codeforces.com/contest/2091/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"8\n9 6\n10 7\n24 2\n123456 777\n6 4\n99 6\n10 4\n99 4\n","output":"4\n1\n2\n775\n1\n4\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_int();
    let k = input.read_int();

    let mut pos = vec![0];
    let mut step = k;
    loop {
        let mut neg = vec![];
        for i in pos.copy_iter() {
            if (s - i) % step == 0 {
                out.print_line(step);
                return;
            }
        }
        for i in pos.copy_iter() {
            let mut at = i;
            for _ in 0..k {
                if at + step > s {
                    break;
                }
                at += step;
                neg.push(at);
            }
        }
        neg.sort();
        neg.dedup();
        pos.clear();
        step -= 1;
        if step == 0 {
            step = 1;
        }
        for i in neg.copy_iter() {
            let mut at = i;
            for _ in 0..k {
                if at < step {
                    break;
                }
                at -= step;
                pos.push(at);
            }
        }
        pos.sort();
        pos.dedup();
        step -= 1;
        if step == 0 {
            step = 1;
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
