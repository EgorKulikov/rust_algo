//{"name":"J. Jolly Wheel","group":"Universal Cup - GP of Belarus","url":"https://contest.ucup.ac/contest/3426/problem/17269","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 4\n4 8 4\n2 7\n6 5\n","output":"4 5\n0 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n);

    let mut at = 0;
    let mut add = 0;
    let mut delta = DefaultTreeMap::new(0);
    delta[0] = 1;
    for i in 0..n {
        let mut cur = a[i] * 2;
        add += cur / (k * 2);
        cur %= k * 2;
        if i % 2 == 0 {
            at += cur;
            if at >= k * 2 {
                at -= k * 2;
                add += 1;
            }
            if i == n - 1 {
                delta[at + 1] -= 1;
            } else {
                delta[at] -= 1;
                delta[at + 1] -= 1;
            }
        } else {
            at -= cur;
            if at < 0 {
                at += k * 2;
                add += 1;
            }
            if i == n - 1 {
                delta[at] += 1;
            } else {
                delta[at] += 1;
                delta[at + 1] += 1;
            }
        }
    }
    let mut cur = add;
    let mut max = 0;
    let mut min = i64::MAX;
    for delta in delta.into_values() {
        cur += delta;
        max.maxim(cur);
        min.minim(cur);
    }
    out.print_line((min, max));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
