//{"name":"E - Building View","group":"AtCoder - AtCoder Weekday Contest 0041 Beta","url":"https://atcoder.jp/contests/awc0041/tasks/awc0041_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n3 1 4 2 5\n1\n2\n3\n5\n","output":"5\n3\n2\n1\n"},{"input":"7 4\n2 7 1 5 3 6 4\n1\n3\n5\n7\n","output":"7\n3\n2\n1\n"},{"input":"10 5\n5 3 8 1 7 2 10 6 4 9\n1\n2\n3\n5\n10\n","output":"10\n6\n4\n2\n1\n"},{"input":"15 6\n10 3 14 7 1 12 5 15 2 11 4 13 6 9 8\n1\n2\n4\n8\n10\n15\n","output":"15\n8\n4\n1\n1\n1\n"},{"input":"1 1\n1000000000\n1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let h = input.read_size_vec(n);

    let mut by_h = DefaultTreeMap::new(Vec::new());
    for i in 0..n {
        by_h[Reverse(h[i])].push(i + 1);
    }
    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(n + 1);
    let mut views = MultiTreapSet::new();
    for pos in by_h.into_values() {
        for i in pos.copy_iter() {
            let left = *set.floor(&i).unwrap();
            let right = *set.ceil(&i).unwrap();
            views.insert(right - left - 1);
        }
        for i in pos {
            set.insert(i);
        }
    }
    for _ in 0..q {
        let k = input.read_size();
        out.print_line(views.more_or_eq(&k));
    }
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
