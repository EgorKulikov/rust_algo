//{"name":"E. Broken Queries","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/E","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n8\n\n0\n\n0\n\n1\n\n0\n\n4\n\n1\n\n0\n","output":"? 3 5\n\n? 1 8\n\n? 4 8\n\n? 3 8\n\n! 6\n\n? 3 3\n\n? 3 4\n\n! 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBrokenQueries"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = Vec::new();
    for i in 0..3 {
        out.print_line(('?', i * n / 4 + 1, (i + 1) * n / 4));
        out.flush();
        ans.push(input.read_int());
    }
    let sum = ans.copy_sum();
    if sum >= 2 {
        ans.push(3 - sum);
    } else {
        ans.push(1 - sum);
    }
    if sum >= 2 {
        let from = if ans[0] == 1 { 1 } else { 1 + n / 4 };
        let mut left = 2;
        let mut right = n / 4;
        while left < right {
            let mid = (left + right) / 2;
            out.print_line(('?', from, from + mid - 1));
            out.flush();
            if input.read_int() == 1 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        out.print_line(('!', left));
        out.flush();
    } else {
        let mut to = 0;
        for i in 0..4 {
            if ans[i] == 1 {
                to = (i + 1) * n / 4;
                break;
            }
        }
        let mut left = n / 4 + 1;
        let mut right = n - 1;
        while left < right {
            let mid = (left + right) / 2;
            let from = to.saturating_sub(mid - 1).max(1);
            let to = from + mid - 1;
            out.print_line(('?', from, to));
            out.flush();
            if input.read_int() == 1 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        out.print_line(('!', left));
        out.flush();
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
