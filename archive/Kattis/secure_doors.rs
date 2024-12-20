//{"name":"Secure Doors","group":"Kattis","url":"https://open.kattis.com/problems/securedoors","interactive":false,"timeLimit":1000,"tests":[{"input":"8\nentry Abbey\nentry Abbey\nexit Abbey\nentry Tyrone\nexit Mason\nentry Demetra\nexit Latonya\nentry Idella\n","output":"Abbey entered\nAbbey entered (ANOMALY)\nAbbey exited\nTyrone entered\nMason exited (ANOMALY)\nDemetra entered\nLatonya exited (ANOMALY)\nIdella entered\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SecureDoors"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut people = FxHashSet::default();
    for _ in 0..n {
        let action = input.read_str();
        let name = input.read_str();

        match action.as_slice() {
            b"entry" => {
                if people.insert(name.clone()) {
                    out.print_line((name, "entered"));
                } else {
                    out.print_line((name, "entered (ANOMALY)"));
                }
            }
            b"exit" => {
                if people.remove(&name) {
                    out.print_line((name, "exited"));
                } else {
                    out.print_line((name, "exited (ANOMALY)"));
                }
            }
            _ => unreachable!(),
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
