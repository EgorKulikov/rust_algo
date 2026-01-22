//{"name":"Nikola","group":"Kattis","url":"https://open.kattis.com/problems/nikola","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n2\n3\n4\n5\n6\n","output":"12\n"},{"input":"8\n2\n3\n4\n3\n1\n6\n1\n4\n","output":"14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let fee = input.read_int_vec(n);

    let mut mem = Memoization2d::new(n, n + 1, |mem, pos, len| {
        if pos == n - 1 {
            return fee[n - 1];
        }
        let mut res = i32::MAX;
        if pos + len + 1 < n {
            res.minim(fee[pos] + mem.call(pos + len + 1, len + 1));
        }
        if pos >= len && len > 0 {
            res.minim(fee[pos] + mem.call(pos - len, len));
        }
        res
    });
    out.print_line(mem.call(0, 0) - fee[0]);
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
