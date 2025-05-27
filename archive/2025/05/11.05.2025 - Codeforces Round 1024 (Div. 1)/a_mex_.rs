//{"name":"A. MEX на матрице","group":"Codeforces - Codeforces Round 1024 (Div. 1)","url":"https://codeforces.com/contest/2101/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2\n3\n","output":"0 1\n2 3\n8 4 5\n6 0 1\n7 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = Arr2d::new(n, n, 0);
    let mut r1 = (n - 1) / 2;
    let mut r2 = (n - 1) / 2;
    let mut c1 = (n - 1) / 2;
    let mut c2 = (n - 1) / 2;
    let mut next = 1;
    for i in 0.. {
        match i % 4 {
            0 => {
                if r2 == n - 1 {
                    break;
                }
                r2 += 1;
                for j in c1..=c2 {
                    ans[(r2, j)] = next;
                    next += 1;
                }
            }
            1 => {
                if c2 == n - 1 {
                    break;
                }
                c2 += 1;
                for j in r1..=r2 {
                    ans[(j, c2)] = next;
                    next += 1;
                }
            }
            2 => {
                if r1 == 0 {
                    break;
                }
                r1 -= 1;
                for j in c1..=c2 {
                    ans[(r1, j)] = next;
                    next += 1;
                }
            }
            3 => {
                if c1 == 0 {
                    break;
                }
                c1 -= 1;
                for j in r1..=r2 {
                    ans[(j, c1)] = next;
                    next += 1;
                }
            }
            _ => unreachable!(),
        }
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
