//{"name":"Constant Subsequence","group":"CodeChef - START169A","url":"https://www.codechef.com/START169A/problems/P5169","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 2 -1 -2\n2\n-1 -1\n4\n-2 3 0 2\n","output":"2\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let pos = a.copy_filter(|&x| x >= 0).collect::<Vec<_>>();
    let neg = a.copy_filter(|&x| x < 0).collect::<Vec<_>>();

    let mut left = pos.copy_iter().max().unwrap_or(0);
    let mut right = pos.copy_sum();
    while left < right {
        let mid = (left + right) / 2;
        let mut cur = 0;
        let mut at = 0;
        let mut good = true;
        for i in pos.copy_iter() {
            while at < neg.len() && cur + i > mid {
                cur = (cur + neg[at]).max(0);
                at += 1;
            }
            if cur + i > mid {
                good = false;
                break;
            }
            cur += i;
        }
        if good {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out.print_line(left);
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
