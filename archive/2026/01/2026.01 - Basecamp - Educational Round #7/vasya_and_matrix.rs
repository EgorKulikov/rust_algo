//{"name":"Vasya and matrix","group":"Eolymp - Basecamp - Educational Round #7","url":"https://eolymp.com/en/compete/0fdopgb6et7g3allmrhbicn9cg/problem/10","interactive":false,"timeLimit":1000,"tests":[{"input":"1 3 4\n8 6 4\n","output":"1\n"},{"input":"3 3 12\n7 5 7\n8 4 8\n4 3 2\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_int();
    let s = (0..n)
        .map(|_| input.read_int_vec(m).partial_sums())
        .collect::<Vec<_>>();

    let mut cur = vec![0; n];
    let mut ans = 0;
    for i in 0..m {
        for j in i + 1..=m {
            for k in 0..n {
                cur[k] = s[k][j] - s[k][i];
            }
            let mut front = 0;
            let mut sum = 0;
            for back in 0..n {
                sum += cur[back];
                while sum > k {
                    sum -= cur[front];
                    front += 1;
                }
                ans.maxim((back + 1 - front) * (j - i));
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
