//{"name":"Fifty Shades of Pink","group":"Kattis","url":"https://open.kattis.com/problems/fiftyshades","interactive":false,"timeLimit":1000,"tests":[{"input":"12\npink\ntequilaSunrose\nmExicanPInK\nCoquelicot\nturqrose\nroSee\nJETblack\npink\nbabypink\npInKpinkPinK\nPInkrose\nlazerlemon\n","output":"9\n"},{"input":"6\nroose\nrosse\npinnk\nscrew\ncerise\nmagenta\n","output":"I must watch Star Wars with my daughter\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FiftyShadesOfPink"}}}

use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::string_algorithms::string_search::StringSearch;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let ans = input
        .iter::<Str>()
        .take(n)
        .filter(|s| {
            s.to_ascii_lowercase().str_contains(b"pink")
                || s.to_ascii_lowercase().str_contains(b"rose")
        })
        .count();
    if ans > 0 {
        out.print_line(ans);
    } else {
        out.print_line("I must watch Star Wars with my daughter");
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
