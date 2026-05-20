//{"name":"Charting Progress","group":"Kattis","url":"https://open.kattis.com/problems/chartingprogress","interactive":false,"timeLimit":1000,"tests":[{"input":"...........*........\n....*.....*.........\n.........*..*...*...\n*..*..*......***....\n..*.....*...........\n.*..................\n.......*.........*.*\n....................\n.....*............*.\n\n..........\n.*.**.*...\n*....*.*.*\n..........\n..*.....*.\n","output":"...................*\n.................**.\n..............***...\n........******......\n......**............\n.....*..............\n..***...............\n....................\n**..................\n\n..........\n......****\n..****....\n..........\n**........\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut s = Vec::new();
    loop {
        if input.is_exhausted() || input.peek().unwrap() == b'\n' {
            input.get();
            break;
        }
        s.push(input.read_line());
    }

    let s = Arr2d::with_gen(s.len(), s[0].len(), |i, j| s[i][j]);
    let pos = Vec::with_gen(s.d2(), |i| s.col(i).position(|&x| x == b'*').unwrap_or(0));
    let order = pos
        .indices()
        .collect::<Vec<_>>()
        .sorted_by_key(|&i| Reverse(pos[i]));
    let ans = Arr2d::with_gen(s.d1(), s.d2(), |i, j| s[(i, order[j])]);
    out.print_table(&ans);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
