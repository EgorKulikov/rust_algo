//{"name":"Lyklagangriti","group":"Kattis","url":"https://open.kattis.com/problems/lyklagangriti","interactive":false,"timeLimit":1000,"tests":[{"input":"iLnLnLeLb\n","output":"benni\n"},{"input":"arnarLLLBBun\n","output":"unnar\n"},{"input":"password123\n","output":"password123\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Lyklagangriti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let mut front = Vec::new();
    let mut back = Vec::new();
    for c in s {
        match c {
            b'L' => {
                back.push(front.pop().unwrap());
            }
            b'R' => {
                front.push(back.pop().unwrap());
            }
            b'B' => {
                front.pop();
            }
            _ => {
                front.push(c);
            }
        }
    }
    back.reverse();
    output!(out, "{}{}", Str::from(front), Str::from(back));
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
