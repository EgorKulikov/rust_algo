//{"name":"T596591 [语言月赛 202504] 整除判断","group":"Luogu","url":"https://www.luogu.com.cn/problem/T596591?contestId=240317","interactive":false,"timeLimit":1000,"tests":[{"input":"6 5\n12\n24\n33\n165\n8\n","output":"33\n165\n"},{"input":"8 2\n15\n9\n","output":"None\n"},{"input":"7 5\n142\n106\n7777\n1000006\n25\n","output":"142\n106\n25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::sum_digs;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_int();
    let n = input.read_size();

    let mut found = false;
    for _ in 0..n {
        let x = input.read_int();
        if x % m != 0 && sum_digs(x) % m == 0 {
            out.print_line(x);
            found = true;
        }
    }
    if !found {
        out.print_line("None");
    }
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
