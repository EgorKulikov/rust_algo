//{"name":"problem_f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::str_scan;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let k = input.read_size();

    for j in 0..s.len() - 1 {
        str_scan!(&s[..=j], "@", num: usize);
        if num >= k {
            // Solve
            let mut num = num;
            let mut ans = Vec::new();
            while num >= k - ans.len() + 9 {
                ans.push(10);
                num -= 10;
            }
            while ans.len() + 1 < k {
                ans.push(1);
                num -= 1;
            }
            ans.reverse();
            out.print(num);
            out.print(Str::from(&s[j + 1..]));
            for i in ans {
                out.print(b' ');
                out.print(i);
                for _ in 0..s.len() - j - 1 {
                    out.print(b'0');
                }
            }
            out.print_line(());
            return;
        }
    }
    str_scan!(&s, "@", a: usize);
    let mut a = a;
    let mut ans = Vec::new();
    while a >= 10 {
        ans.push(10);
        a -= 10;
    }
    if a > 0 {
        ans.push(a);
        ans.reverse();
    }
    while ans.len() < k {
        ans.push(0);
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
