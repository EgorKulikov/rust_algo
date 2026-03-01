//{"name":"Product Minimize Mod","group":"Eolymp - Basecamp - Blitz Round #14","url":"https://eolymp.com/en/compete/37pbmqipe915d0nhjqhvk0ibno/problem/5","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 13\n2 5 7 3\n4 8 9 5\n3 10\n2 4 6\n3 5 7\n7 20\n1 2 3 4 5 6 7\n1 2 3 4 5 6 7\n8 9\n1 2 3 4 5 6 7 8\n2 3 3 4 5 7 8 8\n","output":"3 8 8 4\n2 5 7\n1 2 3 4 5 6 7\n2 3 3 4 5 6 7 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let c = input.read_size();
    let l = input.read_size_vec(n);
    let r = input.read_size_vec(n);

    let mut mem = Memoization2d::new(n + 1, c, |mem, step, rem| -> (usize, usize) {
        if step == n {
            (rem, 0)
        } else {
            let mut ans = (c, 0);
            for i in l[step]..=r[step].min(l[step] + c - 1) {
                let res = mem.call(step + 1, rem * i % c).0;
                ans.minim((res, i));
            }
            ans
        }
    });
    let mut rem = 1;
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let cur = mem.call(i, rem).1;
        ans.push(cur);
        rem = rem * cur % c;
    }
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
