//{"name":"E - Peak Collection","group":"AtCoder - AtCoder Weekday Contest 0018 Beta","url":"https://atcoder.jp/contests/awc0018/tasks/awc0018_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 10\n3 100\n2 200\n5 150\n4 300\n1 400\n","output":"3\n"},{"input":"4 3 3\n2 100\n2 200\n2 300\n2 400\n","output":"1\n"},{"input":"10 5 20\n3 500\n2 100\n5 300\n1 200\n4 800\n2 600\n3 150\n6 900\n1 700\n5 1000\n","output":"5\n"},{"input":"20 10 50\n5 320\n3 150\n7 410\n2 80\n4 500\n6 230\n1 600\n8 90\n3 700\n5 350\n2 850\n4 120\n6 950\n1 40\n3 1100\n7 560\n2 1200\n5 780\n4 1350\n3 999\n","output":"10\n"},{"input":"1 1 1\n1 1000000000\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let b = input.read_size();
    let cs = input.read_size_pair_vec(n);

    let (c, s) = cs.detuple();
    let mut mem = Memoization2d::new(n, b, |mem, i, rem| {
        let mut res = 1;
        for j in i + 1..n {
            if s[j] > s[i] && c[j] <= rem {
                res.maxim(1 + mem.call(j, rem - c[j]));
            }
        }
        res
    });
    let mut ans = 0;
    for i in 0..n {
        ans.maxim(mem.call(i, b - c[i]));
    }
    out.print_line(ans.min(k));
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
