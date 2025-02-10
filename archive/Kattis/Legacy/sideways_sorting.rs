//{"name":"Sideways Sorting","group":"Kattis","url":"https://open.kattis.com/problems/sidewayssorting","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\noTs\nnwi\neox\n3 4\nxAxa\nyByb\nzCyc\n0 0\n","output":"osT\nniw\nexo\n\nAaxx\nBbyy\nCcyz\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let r = input.read_size();
    let c = input.read_size();
    let table = input.read_char_table(r, c);

    if r == 0 {
        return;
    }

    let strings = (0..c)
        .map(|j| table.col(j).copied().collect::<Str>())
        .collect::<Vec<_>>()
        .sorted_by_key(|s| s.to_ascii_lowercase());

    if test_case > 1 {
        out.print_line(());
    }
    for i in 0..r {
        for j in 0..c {
            out.print(strings[j][i]);
        }
        out.print_line(());
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
