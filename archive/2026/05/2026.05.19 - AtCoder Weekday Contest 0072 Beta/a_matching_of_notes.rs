//{"name":"A - Matching of Notes","group":"AtCoder - AtCoder Weekday Contest 0072 Beta","url":"https://atcoder.jp/contests/awc0072/tasks/awc0072_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5\nab?de\na?cd?\n","output":"abcde\nNo\n"},{"input":"6\nabc?ef\naxc?yf\n","output":"a!c?!f\nYes\n"},{"input":"15\nhe??o?wor?d?abc\n?ell??world?axc\n","output":"hello?world?a!c\nYes\n"},{"input":"40\nab?dab?dab?dab?dab?dab?dab?dab?dab?dab?d\na?cda?cda?cda?cda?cda?cda?cda?cda?cda?cd\n","output":"abcdabcdabcdabcdabcdabcdabcdabcdabcdabcd\nNo\n"},{"input":"1\n?\n?\n","output":"?\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let t = input.read_str();

    let ans = Str::from(Vec::with_gen(n, |i| {
        if s[i] == t[i] {
            s[i]
        } else if s[i] == b'?' {
            t[i]
        } else if t[i] == b'?' {
            s[i]
        } else {
            b'!'
        }
    }));
    out.print_line(&ans);
    out.print_line(ans.contains(&b'!'));
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
