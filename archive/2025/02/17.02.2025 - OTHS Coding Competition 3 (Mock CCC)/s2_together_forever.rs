//{"name":"S2 - Together Forever","group":"DMOJ - OTHS Coding Competition 3 (Mock CCC)","url":"https://dmoj.ca/problem/othscc3p2","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3 1 2\n1 2 3\n","output":"2\n1 2\n2 3\n"},{"input":"5\n2 2 1 1 3\n1 1 3 2 2\n","output":"3\n3 5\n1 5\n2 4\n"},{"input":"3\n1 1 1\n1 1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n).dec();
    let b = input.read_size_vec(n).dec();

    let mut inv = vec![BTreeSet::new(); n];
    for i in 0..n {
        inv[a[i]].insert(i);
    }
    let mut ans = Vec::new();
    for i in 0..n {
        if a[i] != b[i] {
            let pos = *inv[b[i]].iter().next().unwrap();
            ans.push((i, pos));
            inv[b[i]].remove(&pos);
            inv[a[i]].remove(&i);
            inv[a[i]].insert(pos);
            a.swap(i, pos);
        } else {
            inv[a[i]].remove(&i);
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans.inc());
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
