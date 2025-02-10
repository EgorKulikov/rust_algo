//{"name":"Undead or Alive","group":"Kattis","url":"https://open.kattis.com/problems/undeadoralive","interactive":false,"timeLimit":1000,"tests":[{"input":"Hello, how are you? :)\n","output":"alive\n"},{"input":"Hey there! :( What's up? :(\n","output":"undead\n"},{"input":"::(Braaaains... are very useful for programming contests:))\n","output":"double agent\n"},{"input":"Sandy, when will my order be delivered?\n","output":"machine\n"},{"input":"Firing up EmoticonBot... (:  : (  ):  :D  c:\n","output":"machine\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::string_search::StringSearch;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line();

    let smily = s.str_contains(b":)");
    let frowny = s.str_contains(b":(");
    out.print_line(match (smily, frowny) {
        (true, true) => "double agent",
        (true, false) => "alive",
        (false, true) => "undead",
        (false, false) => "machine",
    });
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
