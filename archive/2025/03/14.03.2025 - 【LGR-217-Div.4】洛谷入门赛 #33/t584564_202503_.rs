//{"name":"T584564 [语言月赛 202503] 半个哥德巴赫猜想","group":"Luogu","url":"https://www.luogu.com.cn/problem/T584564?contestId=235262","interactive":false,"timeLimit":1000,"tests":[{"input":"11\n","output":"3\n3\n"},{"input":"27\n","output":"6\n5\n"},{"input":"1925\n","output":"170\n17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::multiplicative_function::MulitplicativeFunction;
use algo_lib::numbers::primes::sieve::primality_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let is_p = primality_table(n + 1);
    let mu = MulitplicativeFunction::mobius().calculate_up_to(n + 1);
    let mut ans = 0;
    let mut diff = None;
    for i in 1..n {
        if is_p[i] && mu[n - i] == 0 {
            ans += 1;
            diff.minim(i.abs_diff(n - i));
        }
    }
    out.print_line(ans);
    out.print_line(diff);
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
