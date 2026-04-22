//{"name":"B - Data Compression","group":"AtCoder - AtCoder Weekday Contest 0051 Beta","url":"https://atcoder.jp/contests/awc0051/tasks/awc0051_b","interactive":false,"timeLimit":2000,"tests":[{"input":"aaabbc\n","output":"a3b2c\n"},{"input":"abcde\n","output":"abcde\n"},{"input":"aaabbbbccdddddeeefghhhh\n","output":"a3b4c2d5e3fgh4\n"},{"input":"zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbccccccccccddddddddddddeeeeeeeeeeffffffffgggggggggggggghhhhhhhhhiiiiiiiiiiijjjjjjjjkkkkkkkkkkklllllllllllmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzz\n","output":"z51a20b15c10d12e10f8g14h9i11j8k11l11m9n10o10p10q10r10s10t10u10v10w10x10y10z10\n"},{"input":"a\n","output":"a\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let mut cur = s[0];
    let mut qty = 0;
    for c in s {
        if c != cur {
            out.print(cur);
            if qty > 1 {
                out.print(qty);
            }
            qty = 0;
            cur = c;
        }
        qty += 1;
    }
    out.print(cur);
    if qty > 1 {
        out.print(qty);
    }
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
