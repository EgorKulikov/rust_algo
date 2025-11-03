//{"name":"B. Make Connected","group":"Codeforces - Pinely Round 5 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2161/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"11\n1\n.\n1\n#\n3\n.##\n.##\n...\n3\n#..\n.#.\n..#\n3\n###\n...\n...\n3\n#.#\n...\n.#.\n4\n####\n#..#\n#..#\n####\n3\n..#\n...\n.#.\n3\n..#\n#..\n...\n5\n#.#.#\n.#.#.\n#.#.#\n.#.#.\n#.#.#\n5\n...#.\n...#.\n.....\n##...\n.....\n","output":"YES\nYES\nYES\nYES\nNO\nNO\nNO\nYES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let g = input.read_char_table(n, n);

    for i in 0..n {
        for j in 0..n {
            let mut good = true;
            for (r, c) in g.indices() {
                if g[(r, c)] == b'#' && (r < i || r >= i + 2 || c < j || c >= j + 2) {
                    good = false;
                    break;
                }
            }
            if good {
                out.print_line(true);
                return;
            }
        }
    }
    for i in 0..2 * n - 2 {
        let mut good = true;
        for (r, c) in g.indices() {
            if g[(r, c)] == b'#' && (r + c < i || r + c >= i + 2) {
                good = false;
                break;
            }
        }
        if good {
            out.print_line(true);
            return;
        }
    }
    for i in 0..2 * n - 2 {
        let mut good = true;
        for (r, c) in g.indices() {
            if g[(r, c)] == b'#' && (r + n - 1 - c < i || r + n - 1 - c >= i + 2) {
                good = false;
                break;
            }
        }
        if good {
            out.print_line(true);
            return;
        }
    }
    out.print_line(false);
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
