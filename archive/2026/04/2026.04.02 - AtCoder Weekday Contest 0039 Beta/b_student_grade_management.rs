//{"name":"B - Student Grade Management","group":"AtCoder - AtCoder Weekday Contest 0039 Beta","url":"https://atcoder.jp/contests/awc0039/tasks/awc0039_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 7 60\n1 50\n1 40\n2 80\n2 70\n3 55\n4 60\n4 65\n","output":"2\n"},{"input":"8 10 50\n1 50\n1 50\n2 49\n2 51\n3 0\n3 0\n4 100\n5 30\n5 20\n6 49\n","output":"3\n"},{"input":"10 15 70\n1 90\n1 80\n1 70\n2 60\n2 50\n3 100\n4 65\n4 70\n4 68\n6 40\n6 30\n7 70\n8 69\n8 71\n9 0\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let t = input.read_size();
    let cs = input.read_size_pair_vec(m);

    let mut total = vec![0; n];
    let mut qty = vec![0; n];
    for (c, s) in cs {
        total[c - 1] += s;
        qty[c - 1] += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        if qty[i] * t > total[i] {
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
