//{"name":"Un-bear-able Zoo","group":"Kattis","url":"https://open.kattis.com/problems/zoo","interactive":false,"timeLimit":1000,"tests":[{"input":"6\nAfrican elephant\nWhite tiger\nIndian elephant\nSiberian tiger\nTiger\nPanda bear\n1\nBlue Russian Penguin\n0\n","output":"List 1:\nbear | 1\nelephant | 2\ntiger | 3\nList 2:\npenguin | 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"UnBearAbleZoo"}}}

use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;
use algo_lib::string::split::StrSplit;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let animals = input.read_line_vec(n);

    if n == 0 {
        return;
    }
    let mut qty = DefaultTreeMap::new(0);
    for animal in animals {
        qty[Str::from(animal.str_split(b" ").last().unwrap().to_ascii_lowercase())] += 1;
    }

    output!(out, "List {}:", test_case);
    for (k, v) in qty {
        output!(out, "{} | {}", k, v);
    }
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
