//{"name":"B - Investigation of Friend Relationships","group":"AtCoder - AtCoder Weekday Contest 0013 Beta","url":"https://atcoder.jp/contests/awc0013/tasks/awc0013_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n1 2\n2 1\n1 3\n3 4\n4 3\n","output":"2\n"},{"input":"3 6\n1 2\n2 1\n2 3\n3 2\n1 3\n3 1\n","output":"3\n"},{"input":"10 12\n1 2\n2 1\n3 4\n4 3\n5 6\n6 5\n7 8\n1 3\n2 4\n9 10\n10 9\n8 7\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let m = input.read_size();
    let ab = input.read_size_pair_vec(m);

    let mut seen = FxHashSet::default();
    let mut ans = 0;
    for (a, b) in ab {
        if seen.contains(&(b, a)) {
            ans += 1;
        }
        seen.insert((a, b));
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
