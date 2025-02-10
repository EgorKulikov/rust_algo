//{"name":"This Ain't Your Grandpa's Checkerboard","group":"Kattis","url":"https://open.kattis.com/problems/thisaintyourgrandpascheckerboard","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nWBBW\nWBWB\nBWWB\nBWBW\n","output":"1\n"},{"input":"4\nBWWB\nBWBB\nWBBW\nWBWW\n","output":"0\n"},{"input":"6\nBWBWWB\nWBWBWB\nWBBWBW\nBBWBWW\nBWWBBW\nWWBWBB\n","output":"0\n"},{"input":"6\nWWBBWB\nBBWWBW\nWBWBWB\nBWBWBW\nBWBBWW\nWBWWBB\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let grid = input.read_char_table(n, n);

    fn check<'a>(iter: impl Iterator<Item = &'a u8>) -> bool {
        let mut count_w = 0;
        let mut count_b = 0;
        let mut count = 0;
        let mut last = b' ';
        for &c in iter {
            if c != last {
                count = 0;
                last = c;
            }
            count += 1;
            if count >= 3 {
                return false;
            }
            if c == b'W' {
                count_w += 1;
            } else {
                count_b += 1;
            }
        }
        count_w == count_b
    }
    for i in 0..n {
        if !check(grid.row(i)) || !check(grid.col(i)) {
            out.print_line(0);
            return;
        }
    }
    out.print_line(1);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
