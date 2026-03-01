//{"name":"University Promotion","group":"Eolymp - Basecamp - Blitz Round #14","url":"https://eolymp.com/en/compete/37pbmqipe915d0nhjqhvk0ibno/problem/3","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 4\n90 85 85 70 60 50\n3 1 2 3\n2 1 2\n3 2 3 4\n1 1\n2 3 4\n1 4\n80 80 60 40\n1 1 2 1\n5 5\n100 95 90 85 80\n5 1 2 3 4 5\n1 1\n3 2 3 4\n2 3 4\n1 5\n90 80 90 100 100\n2 1 1 1 1\n","output":"1 2 3 -1 4 -1\n1 1 2 -1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let p = input
        .iter::<Vec<usize>>()
        .take(n)
        .map(|v| v.dec())
        .collect::<Vec<_>>();
    let b = input.read_int_vec(m);
    let mut c = input.read_size_vec(m);

    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| Reverse(a[i]));
    let mut ans = vec![None; n];
    for i in order {
        for j in p[i].copy_iter() {
            if c[j] > 0 && b[j] < a[i] {
                ans[i] = Some(j + 1);
                c[j] -= 1;
                break;
            }
        }
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
