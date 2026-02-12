//{"name":"D - Parking Lot Assignment","group":"AtCoder - AtCoder Weekday Contest 0004 Beta","url":"https://atcoder.jp/contests/awc0004/tasks/awc0004_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 2\n2 3\n1 3\n","output":"Yes\n"},{"input":"4 5\n1 2\n1 2\n3 4\n3 4\n2 3\n","output":"No\n"},{"input":"10 7\n1 5\n2 6\n3 7\n1 3\n5 8\n7 10\n8 10\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let lr = input.read_size_pair_vec(m).dec().sorted_by_key(|&(_, r)| r);

    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert(i);
    }
    for (l, r) in lr {
        if let Some(&x) = set.ceil(&l) {
            if x <= r {
                set.remove(&x);
            } else {
                out.print_line(false);
                return;
            }
        } else {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
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
