//{"name":"C. Kevin and Binary Strings","group":"Codeforces - Codeforces Global Round 28","url":"https://codeforces.com/contest/2048/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n111\n1000\n10111\n11101\n1100010001101\n","output":"2 2 1 3\n1 3 1 4\n1 5 1 4\n3 4 1 5\n1 13 1 11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKevinAndBinaryStrings"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    for i in s.indices() {
        if s[i] == b'0' {
            let mut best = s.clone();
            let mut l2 = 0;
            let mut r2 = 0;
            for j in 0..i {
                let mut cur = s.clone();
                for k in i..s.len() {
                    if s[k - i + j] == b'1' {
                        cur[k] ^= 1;
                    }
                }
                if best.maxim(cur) {
                    l2 = j + 1;
                    r2 = j + s.len() - i;
                }
            }
            out.print_line((1, s.len(), l2, r2));
            return;
        }
    }
    out.print_line((1, s.len(), 1, 1));
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
