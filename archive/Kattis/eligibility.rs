//{"name":"Eligibility","group":"Kattis","url":"https://open.kattis.com/problems/eligibility","interactive":false,"timeLimit":2000,"tests":[{"input":"3\nEligibleContestant 2013/09/01 1995/03/12 10\nIneligibleContestant 2009/09/01 1990/12/12 50\nPetitionContestant 2009/09/01 1990/12/12 35\n","output":"EligibleContestant eligible\nIneligibleContestant ineligible\nPetitionContestant coach petitions\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Eligibility"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    scan!(input, "@ @/@ @/@ @", name: Str, start_year: i32, _start_date: Str, birth_year: i32, _birth_date: Str, courses: i32);

    if start_year >= 2010 || birth_year >= 1991 {
        out.print_line(format!("{} eligible", name));
    } else if courses > 40 {
        out.print_line(format!("{} ineligible", name));
    } else {
        out.print_line(format!("{} coach petitions", name));
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
