//{"name":"Gravity Golf","group":"CodeChef - START239A","url":"https://www.codechef.com/START239A/problems/GRVGOLF","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1111\n5\n00000\n4\n1010\n6\n100111\n","output":"1\n0000\n3\n00010\n00111\n10100\n-1\n6\n000000\n000010\n101100\n001010\n100100\n000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    if s[n - 1] != s[n - 2] || s[n - 3] == b'0' && s[n - 2] == b'1' {
        out.print_line(-1);
        return;
    }
    let mut ans = Arr2d::new((3 * n).upper_div(2), n, b'0');
    let mut pos = ans.d1() - 2;
    let mut last = true;
    for i in 0..n - 1 {
        if s[i] == b'0' {
            ans[(pos, i + 1)] = b'1';
            last = false;
        } else {
            if !last {
                pos -= 1;
                ans[(pos, i + 1)] = b'1';
                pos -= 2;
            }
            last = true;
        }
    }
    out.print_line(ans.d1());
    out.print_table(&ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
