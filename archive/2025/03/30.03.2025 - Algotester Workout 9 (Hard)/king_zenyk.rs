//{"name":"King Zenyk","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/136317","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1 3\n","output":"#.###\n#..##\n.####\n#####\n#####\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).sorted().reversed();

    let mut ans = Arr2d::new(100, 100, b'#');
    let mut x = 0;
    let mut y = 0;
    for i in a {
        if i > 100 {
            assert_eq!(y, 0);
            for a in 0..i.upper_div(50) {
                let cur = (i - a * 50).min(50);
                if a % 2 == 0 {
                    for j in (0..2 * cur).step_by(2) {
                        ans[(x, j)] = b'.';
                    }
                } else {
                    for j in (1..2 * cur).step_by(2) {
                        ans[(x, j)] = b'.';
                    }
                }
                x += 2;
            }
            x += 3;
            continue;
        }
        if y + i > 100 {
            x += 5;
            y = 0;
        }
        for _ in 0..i {
            if y % 2 == 0 {
                ans[(x, y)] = b'.';
            } else {
                ans[(x + 2, y)] = b'.';
            }
            y += 1;
        }
        y += 1;
    }
    out.print_table(&ans);
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
