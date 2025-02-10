//{"name":"Quite a Problem","group":"Kattis","url":"https://open.kattis.com/problems/quiteaproblem","interactive":false,"timeLimit":1000,"tests":[{"input":"Problematic pair programming\n\"There's a joke that pairs, like fish and house guests, go\nrotten after three days,\" said Zach Brock, an engineering\nmanager.  Working out problems with a pairing partner can be\na lot like working out problems with a significant other.\nDuring one recent rough patch, Jamie Kite, a developer, sat\nher partner down for a talk. \"Hey, it feels like we're\ndriving in different directions,\" she recalls saying. \"It's\nlike any relationship,\" Ms.  Kite said. \"If you don't talk\nabout the problems, it's not going to work.\" When those\ntimeouts don't solve the problem, partners can turn to\non-staff coaches who can help with counseling.  \"People who\nhave been pairing a while, they'll start acting like old\nmarried couples,\" said Marc Phillips, one of the coaches.\nPeople can be as much of a challenge as writing software.\n(Excerpted from \"Computer Programmers Learn Tough Lesson in\nSharing\"; Wall Street Journal, August 27, 2012)\n","output":"yes\nno\nno\nyes\nyes\nno\nno\nno\nno\nyes\nyes\nno\nno\nno\nno\nno\nno\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::string_search::StringSearch;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line();

    out.print_line(s.to_ascii_lowercase().str_contains(b"problem"));
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::Custom("yes", "no"));

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
