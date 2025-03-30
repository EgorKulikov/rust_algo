//{"name":"G. Lexicographic Raffle","group":"Universal Cup - The 3rd Universal Cup. Stage 33: India","url":"https://contest.ucup.ac/contest/1954/problem/10271","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n6 3\nkanpur\n1 6\n4 6\n5 6\n10 2\nadccbabbab\n1 10\n2 9\n5 3\naccaa\n1 5\n2 4\n3 5\n5 1\njddda\n3 4\n","output":"2\n4\n6\n1\n6\n1\n4\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    let next = Vec::with_gen_back(n, |i, next| {
        if i == n - 1 {
            n - 1
        } else {
            if s[i] < s[i + 1] {
                i
            } else {
                next[i + 1]
            }
        }
    });
    let back = Vec::with_gen_prefix(n, |i, back| {
        if i == 0 {
            0
        } else {
            if s[i] == s[i - 1] {
                back[i - 1]
            } else {
                i
            }
        }
    });
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size() - 1;
        if next[l] < r {
            out.print_line(back[next[l]].max(l) + 1);
        } else {
            out.print_line(back[r].max(l) + 1);
        }
    }
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
