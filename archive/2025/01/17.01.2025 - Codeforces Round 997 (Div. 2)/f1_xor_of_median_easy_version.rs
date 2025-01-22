//{"name":"F1. Xor of Median (Easy Version)","group":"Codeforces - Codeforces Round 997 (Div. 2)","url":"https://codeforces.com/contest/2056/problem/F1","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 3\n10\n2 3\n11\n5 1\n11101\n7 9\n1101011\n17 34\n11001010001010010\n1 500\n1\n","output":"3\n2\n0\n8\n32\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _k = input.read_size();
    let m = input.read_size();
    let n = input.read_str();

    let ones = n.copy_count(b'1');

    let mut ans = 0;
    let mut qq = vec![false; ones + 1];
    qq[0] = true;
    for _ in 0..ones {
        for j in (1..=ones).rev() {
            if j % 2 == 0 {
                qq[j] = qq[j - 1];
            } else {
                qq[j] = qq[j - 1] ^ qq[j];
            }
        }
        qq[0] = false;
    }
    for i in 1..=ones {
        if !qq[i] {
            continue;
        }
        for x in i - 1..m {
            if (x & (i - 1)) == i - 1 {
                ans ^= x;
            }
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

//START MAIN
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
//END MAIN
