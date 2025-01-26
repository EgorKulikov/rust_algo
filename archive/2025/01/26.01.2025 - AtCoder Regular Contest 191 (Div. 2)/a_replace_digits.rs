//{"name":"A - Replace Digits","group":"AtCoder - AtCoder Regular Contest 191 (Div. 2)","url":"https://atcoder.jp/contests/arc191/tasks/arc191_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n191\n325\n","output":"593\n"},{"input":"3 9\n191\n998244353\n","output":"993\n"},{"input":"11 13\n31415926535\n2718281828459\n","output":"98888976555\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut s = input.read_str();
    let t = input.read_str();

    let mut map = by_index(t.as_slice());
    let mut last_used = false;
    for i in 0..n {
        for j in (s[i] + 1..=b'9').rev() {
            if let Some(pos) = map[j].pop() {
                if pos == m - 1 {
                    last_used = true;
                }
                s[i] = j;
                break;
            }
        }
    }
    if !last_used {
        let last = t[m - 1];
        for i in 0..n {
            if s[i] == last {
                last_used = true;
            }
        }
        if !last_used {
            s[n - 1] = last;
        }
    }
    out.print_line(s);
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

//START MAIN
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
//END MAIN
