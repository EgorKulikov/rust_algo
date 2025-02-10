//{"name":"Binary search tree","group":"Kattis","url":"https://open.kattis.com/problems/bst","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n2\n3\n4\n","output":"0\n1\n3\n6\n"},{"input":"5\n3\n2\n4\n1\n5\n","output":"0\n1\n2\n4\n6\n"},{"input":"8\n3\n5\n1\n6\n8\n7\n2\n4\n","output":"0\n1\n2\n4\n7\n11\n13\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut bt = BTreeSet::new();
    let mut h = vec![0; n];
    let mut c = 0;
    for i in 0..n {
        let key = (a[i], i);
        let mut res: usize = 0;
        if let Some(&(_, j)) = bt.prev(&key) {
            res.maxim(h[j] + 1);
        }
        if let Some(&(_, j)) = bt.next(&key) {
            res.maxim(h[j] + 1);
        }
        h[i] = res;
        c += res;
        out.print_line(c);
        bt.insert(key);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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

//START MAIN
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
//END MAIN
