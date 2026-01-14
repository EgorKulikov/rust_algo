//{"name":"H. Hiking","group":"Universal Cup - CERC 2025","url":"https://contest.ucup.ac/contest/2814/problem/16007","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6\n10 18 12 13 17 14\n4\n10 10 7 7\n8\n8 4 8 5 4 4 8 6\n","output":"0\n10 12 13 14 17 18\n1\n7 10 7 10\n4\n4 4 4 5 6 8 8 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut q = DefaultTreeMap::new(0);
    for i in a.copy_iter() {
        q[i] += 1;
    }
    let mut ans = Vec::new();
    let mut loss = 0;
    while !q.is_empty() {
        let mut next = DefaultTreeMap::new(0);
        loss += 1;
        for (k, v) in q {
            if next[k / 2] == 0 {
                ans.push(k);
                if v != 1 {
                    next[k] += v - 1;
                }
            } else {
                next[k] += v;
            }
        }
        q = next;
    }
    loss -= 1;
    out.print_line(loss);
    out.print_line(ans);
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
