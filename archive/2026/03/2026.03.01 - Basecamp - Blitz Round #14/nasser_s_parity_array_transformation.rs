//{"name":"Nasser's Parity Array Transformation","group":"Eolymp - Basecamp - Blitz Round #14","url":"https://eolymp.com/en/compete/37pbmqipe915d0nhjqhvk0ibno/problem/4","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4\n1 3 2\n4 2\n1 2 5 6\n6 9\n3 4 2 1 4 5\n2 4\n1 1\n5 5\n5 5 5 5 5\n","output":"4\n-1\n9\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let a = input.read_long_vec(n);

    if a.copy_max() > x {
        out.print_line(-1);
        return;
    }
    let mut set = (0..n).collect::<BTreeSet<_>>();
    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| a[i]);
    let mut ans = 0;
    for i in order {
        let mut target = x;
        if let Some(&j) = set.prev(&i) {
            target.minim(a[j]);
        }
        if let Some(&j) = set.next(&i) {
            target.minim(a[j]);
        }
        set.remove(&i);
        ans += target - a[i];
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
