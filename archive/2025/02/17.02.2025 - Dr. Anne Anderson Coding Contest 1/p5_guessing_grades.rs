//{"name":"P5 - Guessing Grades","group":"DMOJ - Dr. Anne Anderson Coding Contest 1","url":"https://dmoj.ca/problem/daacc1p5","interactive":false,"timeLimit":1000,"tests":[{"input":"9\nDBCADDACB\nD?C???BC?\n1 1 2 5\n","output":"4\n"},{"input":"12\nBBCCCBAABAAA\nBBCCABCBBABA\n3 6 3 0\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let t = input.read_str();
    let mut tq = input.read_size_vec(4);

    let mut sq = vec![0; 4];
    let mut ans = 0;
    for i in 0..n {
        if t[i] == b'?' {
            let id = (s[i] - b'A') as usize;
            sq[id] += 1;
        } else {
            let id = (t[i] - b'A') as usize;
            tq[id] -= 1;
            if s[i] == t[i] {
                ans += 1;
            }
        }
    }
    let sum = sq.copy_sum();
    for i in 0..4 {
        if sq[i] + tq[i] > sum {
            ans += sq[i] + tq[i] - sum;
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
