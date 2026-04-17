//{"name":"Bread Sorting","group":"Eolymp - Basecamp - Educational Round #10","url":"https://eolymp.com/en/compete/lce8ibks056gvennio8eubldr0/problem/8","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 3 4 2\n4 3 2 1\n","output":"Possible\n"},{"input":"6\n1 2 3 4 5 6\n6 5 4 3 2 1\n","output":"Impossible\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();
    let q = input.read_size_vec(n).dec();

    fn num_inv(p: &[usize]) -> usize {
        let mut ans = 0;
        let mut ft = FenwickTree::new(p.len());
        for &x in p {
            ans += ft.get(x..);
            ft.add(x, 1);
        }
        ans
    }

    out.print_line(num_inv(&p) % 2 == num_inv(&q) % 2);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

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
