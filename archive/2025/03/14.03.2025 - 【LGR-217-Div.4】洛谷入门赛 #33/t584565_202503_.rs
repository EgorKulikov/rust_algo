//{"name":"T584565 [语言月赛 202503] 数字棋盘","group":"Luogu","url":"https://www.luogu.com.cn/problem/T584565?contestId=235262","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n2 3 1\n1 3 1\n2 1 2\n1 2\n","output":"3\n"},{"input":"1 1\n1\n1 1\n","output":"0\n"},{"input":"5 5\n4 4 2 3 1\n1 1 4 3 4\n3 4 2 1 1\n3 1 1 3 3\n4 3 1 3 1\n4 3\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_table(n, m);
    let x = input.read_int();
    let y = input.read_int();

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if a[(i, j)] != x {
                continue;
            }
            let mut found = false;
            for (r, c) in D4::iter(i, j, n, m) {
                if a[(r, c)] == y {
                    found = true;
                    break;
                }
            }
            if found {
                ans += 1;
            }
        }
    }
    out.print_line(ans);
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
