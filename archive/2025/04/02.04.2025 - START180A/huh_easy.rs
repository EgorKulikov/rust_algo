//{"name":"Huh, Easy","group":"CodeChef - START180A","url":"https://www.codechef.com/START180A/problems/HHEY","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 0\n3 0\n3 3\n4 3\n","output":"A\nB\n-1\nABC\nABC\nABCA\nACBA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    if k < n / 2 {
        out.print_line(-1);
        return;
    }
    let mut s = Str::new();
    let mut t = Str::new();
    let mut rem = k;
    for i in 0..n {
        if i % 2 == 0 {
            s.push(b'B');
            t.push(b'C');
        } else {
            s.push(b'A');
            t.push(b'A');
            rem -= 1;
        }
    }
    for i in (0..n).step_by(2) {
        if rem > 0 {
            t[i] = b'B';
            rem -= 1;
        }
    }
    out.print_line(s);
    out.print_line(t);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
