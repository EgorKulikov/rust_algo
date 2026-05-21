use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_4d::Memoization4d;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let mut mem = Memoization4d::new(
        s.len() + 1,
        2,
        4,
        4,
        |mem, pos, has_l, vowels, consonants| -> i64 {
            if vowels == 3 || consonants == 3 {
                0
            } else if pos == s.len() {
                if has_l == 1 {
                    1
                } else {
                    0
                }
            } else if s[pos] == b'_' {
                mem.call(pos + 1, has_l, vowels + 1, 0) * 5
                    + mem.call(pos + 1, has_l, 0, consonants + 1) * 20
                    + mem.call(pos + 1, 1, 0, consonants + 1)
            } else if s[pos] == b'L' {
                mem.call(pos + 1, 1, 0, consonants + 1)
            } else if s[pos] == b'A'
                || s[pos] == b'E'
                || s[pos] == b'I'
                || s[pos] == b'O'
                || s[pos] == b'U'
            {
                mem.call(pos + 1, has_l, vowels + 1, 0)
            } else {
                mem.call(pos + 1, has_l, 0, consonants + 1)
            }
        },
    );
    out.print_line(mem.call(0, 0, 0, 0));
}

pub static TEST_TYPE: TestType = TestType::Single;
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
