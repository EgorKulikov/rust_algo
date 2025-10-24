//{"name":"C - Concat (X-th)","group":"AtCoder - AtCoder Beginner Contest 416","url":"https://atcoder.jp/contests/abc416/tasks/abc416_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2 6\nabc\nxxx\nabc\n","output":"abcxxx\n"},{"input":"5 5 416\na\naa\naaa\naa\na\n","output":"aaaaaaa\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let x = input.read_size() - 1;
    let s = input.read_str_vec(n);

    let mut res = Vec::new();
    let mut rec = RecursiveFunction2::new(|rec, ss: Str, pos: usize| {
        if pos == k {
            res.push(ss);
            return;
        }
        for s in &s {
            let mut ss = ss.clone();
            ss.extend_from_slice(s);
            rec.call(ss, pos + 1);
        }
    });
    rec.call(Str::new(), 0);
    res.sort();
    out.print_line(&res[x]);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
