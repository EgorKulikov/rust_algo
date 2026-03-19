//{"name":"B - Warehouse Inspection Robot","group":"AtCoder - AtCoder Weekday Contest 0025 Beta","url":"https://atcoder.jp/contests/awc0025/tasks/awc0025_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4 9\n#..#\n.##.\n..#.\nRRDDLUUUL\n","output":"1\n"},{"input":"2 3 8\n###\n..#\nUUURRRDD\n","output":"0\n"},{"input":"6 7 30\n.#..#..\n##...#.\n..#.#..\n.##....\n...###.\n#..#..#\nRRRDDRULDLURRRDDLLUUURRDDDLURD\n","output":"10\n"},{"input":"12 15 60\n#..#...##....#.\n.#....#....##..\n....##..#...#..\n###...#..#.....\n..#..#..##..#..\n.....#.....#..#\n.##..#..#..##..\n...#.....##....\n#....###...#...\n..##..#....#..#\n.#...#..#..#...\n...#..##.....#.\nRRDDLLUUURRRDDLLUUURRRDDLLUUURRRDDLLUUURRRDDLLUUURRRDDLLUUUR\n","output":"46\n"},{"input":"1 1 12\n#\nUDLRUDLRUDLR\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let _n = input.read_size();
    let mut s = input.read_char_table(h, w);
    let t = input.read_str();

    s[(0, 0)] = b'.';
    let mut r: usize = 0;
    let mut c: usize = 0;
    for d in t {
        match d {
            b'U' => r = r.saturating_sub(1),
            b'D' => r = (r + 1).min(h - 1),
            b'L' => c = c.saturating_sub(1),
            b'R' => c = (c + 1).min(w - 1),
            _ => unreachable!(),
        }
        if s[(r, c)] == b'#' {
            s[(r, c)] = b'.';
        }
    }
    out.print_line(s.copy_count(b'#'));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
