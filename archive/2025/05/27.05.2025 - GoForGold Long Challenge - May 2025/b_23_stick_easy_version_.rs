//{"name":"B. 23 Stick (Easy Version)","group":"Codeforces - GoForGold Long Challenge - May 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/611602/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2\n5\n","output":"-1\n2 4 1 3 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    if n == 1 {
        out.print_line(1);
        return;
    }
    if n <= 3 {
        out.print_line(-1);
        return;
    }
    if n == 7 {
        out.print_line([1, 3, 6, 4, 7, 5, 2]);
        return;
    }

    let mut ans = Vec::with_capacity(n);
    let even = n - n % 2;
    let mut next = 1;
    if even % 4 == 2 {
        ans.extend_from_slice(&[1, 3, 5, 2, 4, 6]);
        next = 7;
    }
    while even >= next {
        ans.extend_from_slice(&[next + 1, next + 3, next, next + 2]);
        next += 4;
    }
    assert_eq!(next, even + 1);
    if n % 2 == 1 {
        ans.push(n);
    }
    out.print_line(ans);
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
