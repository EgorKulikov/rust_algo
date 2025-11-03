//{"name":"halloween_in_ascii_part_2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let l = input.read_size();
    let r = input.read_size();
    let g = input.read_line_vec(r - l + 1);
    input.read_line();
    let mut image = Vec::new();
    loop {
        let line = input.read_line();
        if line.as_slice() == b"=====" {
            break;
        }
        image.push(line);
    }
    let mut s = Str::new();
    for i in 0..g[0].len() {
        for j in 0..g[0].len() {
            let mut ok = true;
            for k in l..=r {
                if g[k - l][i] != image[k][j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                s.push(image[0][j]);
                break;
            }
        }
    }
    let pos = s.iter().position(|&c| c == b'>').unwrap();
    let to = s[pos..].iter().position(|&c| c == b'<').unwrap() + pos;
    let ans = Str::from(&s[pos + 1..to]);
    writeln!(out, "Case #{}: {}", test_case, ans).unwrap();
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
