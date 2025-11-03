//{"name":"J. Judging Papers","group":"Universal Cup - GP of Chengdu","url":"https://contest.ucup.ac/contest/2567/problem/14715","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 3 2 1\n-3 0 3\n2 -2 -1\n1 1 1\n0 0 0\n-1 -1 -1\n3 2 -1 1\n-1 -2\n-3 -3\n1 -3\n","output":"2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_int();
    let b = input.read_size();
    let s = input.read_int_table(n, m);

    let mut ans = 0;
    let mut add = 0;
    for i in 0..n {
        let sum = s.row(i).copied().sum::<i32>();
        if sum >= k {
            ans += 1;
            continue;
        }
        let less = s.row(i).copied().filter(|&x| x <= 0).count();
        let more = m - less;
        if sum + less as i32 - more as i32 >= k {
            add += 1;
        }
    }
    ans += add.min(b);
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
