//{"name":"Saving Daylight","group":"Kattis","url":"https://open.kattis.com/problems/savingdaylight","interactive":false,"timeLimit":1000,"tests":[{"input":"June 22 2005 6:24 20:37\nDecember 22 2005 7:24 17:30\nNovember 2 2005 6:45 17:38\nJanuary 8 1992 7:45 18:46\n","output":"June 22 2005 14 hours 13 minutes\nDecember 22 2005 10 hours 6 minutes\nNovember 2 2005 10 hours 53 minutes\nJanuary 8 1992 11 hours 1 minutes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::{output, scan};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    scan!(input, "@ @ @ @:@ @:@", month: Str, day: i32, year: i32, sh: i32, sm: i32, eh: i32, em: i32);

    let start = sh * 60 + sm;
    let end = eh * 60 + em;
    let diff = end - start;
    let h = diff / 60;
    let m = diff % 60;
    output!(out, "{} {} {} {} hours {} minutes", month, day, year, h, m);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
