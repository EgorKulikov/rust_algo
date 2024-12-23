//{"name":"Hangman","group":"Kattis","url":"https://open.kattis.com/problems/hangman","interactive":false,"timeLimit":1000,"tests":[{"input":"HANGMAN\nABCDEFGHIJKLMNOPQRSTUVWXYZ\n","output":"WIN\n"},{"input":"BANANA\nABCDEFGHIJKLMNOPQRSTUVWXYZ\n","output":"LOSE\n"},{"input":"RAINBOWS\nUSIANBVLOJRKWXZCTQGHPFMYDE\n","output":"WIN\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Hangman"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let t = input.read_str();

    let mut rem = s.to_vec().sorted().do_with(|v| v.dedup()).len();
    let mut wrong = 0;
    for c in t {
        if s.contains(&c) {
            rem -= 1;
            if rem == 0 {
                out.print_line("WIN");
                return;
            }
        } else {
            wrong += 1;
            if wrong == 10 {
                out.print_line("LOSE");
                return;
            }
        }
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
