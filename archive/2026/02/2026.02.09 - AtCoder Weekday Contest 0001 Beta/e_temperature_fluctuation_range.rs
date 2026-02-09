//{"name":"E - Temperature Fluctuation Range","group":"AtCoder - AtCoder Weekday Contest 0001 Beta","url":"https://atcoder.jp/contests/awc0001/tasks/awc0001_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n2 5 1 8 4\n","output":"7\n"},{"input":"7 4\n-3 10 5 -2 8 1 6\n","output":"13\n"},{"input":"12 5\n100 -50 200 150 -100 300 50 -200 250 0 -150 400\n","output":"600\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::sliding_window::SlidingWindow;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let h = input.read_int_vec(n);

    let mut sw = SlidingWindow::new(k, |(min1, max1): (i32, i32), (min2, max2)| {
        (min1.min(min2), max1.max(max2))
    });
    for i in 0..k - 1 {
        sw.push((h[i], h[i]));
    }
    let mut ans = 0;
    for i in k - 1..n {
        sw.push((h[i], h[i]));
        let (min, max) = sw.get();
        ans.maxim(max - min);
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
