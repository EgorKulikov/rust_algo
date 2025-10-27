//{"name":"F1. Странная операция (простая версия)","group":"Codeforces - Codeforces Round 1061 (Div. 2)","url":"https://codeforces.com/contest/2156/problem/F1","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n4\n3 2 1 4\n5\n3 4 5 2 1\n5\n2 4 5 3 1\n7\n5 3 4 1 2 6 7\n10\n2 7 5 1 3 9 4 10 6 8\n","output":"1 3 2 4\n1 2 3 5 4\n2 4 5 3 1\n1 2 3 4 5 6 7\n2 3 4 1 5 7 6 8 9 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let mut q = p.inv();
    for i in 0..n {
        let mut rec = RecursiveFunction::new(|rec, i: usize| -> (usize, Vec<usize>) {
            if i + 2 >= n {
                (q[i], vec![])
            } else {
                let (to, mut ops) = rec.call(i + 2);
                if to < q[i] && to < q[i + 1] {
                    ops.push(i);
                    (to, ops)
                } else {
                    (q[i], vec![])
                }
            }
        });
        let (_, ops) = rec.call(i);
        for at in ops {
            if q[at] > q[at + 1] {
                let q0 = q[at];
                let q1 = q[at + 1];
                let q2 = q[at + 2];
                q[at] = q2;
                q[at + 2] = q1;
                q[at + 1] = q0;
            } else {
                let q0 = q[at];
                let q1 = q[at + 1];
                let q2 = q[at + 2];
                q[at] = q2;
                q[at + 1] = q0;
                q[at + 2] = q1;
            }
        }
    }
    out.print_line(q.inv().inc());
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
