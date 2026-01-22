//{"name":"Turbo","group":"Kattis","url":"https://open.kattis.com/problems/turbo","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n1\n3\n","output":"1\n0\n0\n"},{"input":"5\n5\n4\n3\n2\n1\n","output":"4\n3\n2\n1\n0\n"},{"input":"7\n5\n4\n3\n7\n1\n2\n6\n","output":"4\n2\n3\n0\n2\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let a_inv = a.inv();
    let mut ft = FenwickTree::from(vec![1i32; n].as_slice());
    for i in 0..n {
        if i % 2 == 0 {
            let pos = a_inv[i / 2];
            ft.add(pos, -1);
            out.print_line(ft.get(..pos));
        } else {
            let pos = a_inv[n - (i + 1) / 2];
            ft.add(pos, -1);
            out.print_line(ft.get(pos..));
        }
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
