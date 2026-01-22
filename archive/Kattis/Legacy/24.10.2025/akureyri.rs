//{"name":"Akureyri","group":"Kattis","url":"https://open.kattis.com/problems/akureyri","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nHjalti\nReykjavik\nGunnar\nReykjavik\nBjarki\nAkureyri\nTomas\nReykjavik\nJonas\nAkureyri\n","output":"Akureyri 2\nReykjavik 3\n"},{"input":"2\nBjarki\nAkureyri\nJonas\nAkureyri\n","output":"Akureyri 2\n"},{"input":"2\nSunna\nSelfoss\nSaga\nAkureyri\n","output":"Akureyri 1\nSelfoss 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut map = DefaultHashMap::new(0);
    for _ in 0..n {
        input.read_str();
        map[input.read_str()] += 1;
    }
    out.print_per_line_iter(map.into_iter());
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
