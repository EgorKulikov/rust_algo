//{"name":"Nine Knights","group":"Kattis","url":"https://open.kattis.com/problems/nineknights","interactive":false,"timeLimit":1000,"tests":[{"input":"...k.\n...k.\nk.k..\n.k.k.\nk.k.k\n","output":"invalid\n"},{"input":".....\n...k.\nk.k.k\n.k.k.\nk.k.k\n","output":"valid\n"},{"input":".....\n...k.\nk.k.k\n.k.k.\nk...k\n","output":"invalid\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NineKnights"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::DK;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_char_table(5, 5);

    if t.copy_count(b'k') != 9 {
        out.print_line("invalid");
        return;
    }
    for i in 0..5 {
        for j in 0..5 {
            if t[(i, j)] == b'k' {
                for (r, c) in DK::iter(i, j, 5, 5) {
                    if t[(r, c)] == b'k' {
                        out.print_line("invalid");
                        return;
                    }
                }
            }
        }
    }
    out.print_line("valid");
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
