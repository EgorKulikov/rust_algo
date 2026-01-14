//{"name":"F - Starry Landscape Photo","group":"AtCoder - AtCoder Beginner Contest 436","url":"https://atcoder.jp/contests/abc436/tasks/abc436_f","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 1 4 2\n","output":"12\n"},{"input":"7\n1 2 3 4 5 6 7\n","output":"28\n"},{"input":"20\n15 5 13 17 9 11 20 4 14 16 6 3 8 19 12 7 10 18 2 1\n","output":"627\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let b = input.read_size_vec(n).dec();

    let mut left = FenwickTree::new(n);
    let mut right = FenwickTree::new(n);
    for i in 0..n {
        right.add(i, 1i64);
    }
    let mut ans = 0;
    for i in 0..n {
        left.add(b[i], 1);
        ans += left.get(..=b[i]) * right.get(..=b[i]);
        right.add(b[i], -1);
    }
    out.print_line(ans);
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
