//{"name":"C - Watering the Flower Bed","group":"AtCoder - AtCoder Weekday Contest 0054 Beta","url":"https://atcoder.jp/contests/awc0054/tasks/awc0054_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n10 5 8 12 3\n1 3 4\n2 5 6\n","output":"3\n"},{"input":"7 4\n15 20 10 25 30 5 18\n1 4 8\n3 6 12\n2 5 5\n4 7 10\n","output":"3\n"},{"input":"10 6\n100 50 80 120 60 90 40 110 70 55\n1 5 30\n3 8 25\n6 10 35\n2 4 40\n5 7 20\n1 10 10\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_long_vec(n);

    let mut delta = vec![0; n + 1];
    for _ in 0..m {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let w = input.read_long();
        delta[l] += w;
        delta[r] -= w;
    }
    let mut ans = 0;
    let mut cur = 0;
    for i in 0..n {
        cur += delta[i];
        if cur > s[i] {
            ans += 1;
        }
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
