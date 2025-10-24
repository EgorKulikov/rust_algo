//{"name":"Amazing Trick","group":"Eolymp - Basecamp - Educational Round 1","url":"https://basecamp.eolymp.com/en/compete/1v399r4bst3f1apjnuj8as5pbc/problem/10","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n2 1\n3\n1 2 3\n4\n2 1 4 3\n5\n5 1 4 2 3\n","output":"Impossible\nPossible\n3 1 2\n2 3 1\nPossible\n3 4 2 1\n3 4 2 1\nPossible\n4 1 2 5 3\n3 1 4 5 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::next_permutation::NextPermutation;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use std::collections::VecDeque;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let b = a.inv();
    let mut p = vec![0; n];
    let mut q = vec![0; n];
    let mut unused = (0..n).collect::<VecDeque<_>>();
    const MAX_FIXED: usize = 4;
    for i in 0..n.saturating_sub(MAX_FIXED) {
        loop {
            let x = unused.pop_front().unwrap();
            if x == i || x == b[i] {
                unused.push_back(x);
            } else {
                q[i] = x;
                p[x] = b[i];
                break;
            }
        }
    }
    let mut rem = unused.into_iter().collect::<Vec<_>>().sorted();
    loop {
        let mut good = true;
        for i in rem.indices() {
            let j = n.saturating_sub(MAX_FIXED) + i;
            if rem[i] == j || rem[i] == b[j] {
                good = false;
                break;
            }
            q[j] = rem[i];
            p[rem[i]] = b[j];
        }
        if good {
            out.print_line(true);
            out.print_line(p.inc());
            out.print_line(q.inc());
            return;
        }
        if !rem.next_permutation() {
            break;
        }
    }
    assert!(rem.len() < MAX_FIXED);
    out.print_line(false);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    output.set_bool_output(BoolOutput::PossibleImpossible);

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
