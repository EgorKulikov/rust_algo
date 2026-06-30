use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    let zeroes = Vec::with_gen(n - 1, |i| {
        if s[i] == b'0' && s[i + 1] == b'0' {
            1
        } else {
            0
        }
    });
    let ones = Vec::with_gen(n - 1, |i| {
        if s[i] == b'1' && s[i + 1] == b'1' {
            1
        } else {
            0
        }
    });
    let sz = zeroes.partial_sums();
    let so = ones.partial_sums();

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size() - 1;

        if sz[r] - sz[l] == r - l || so[r] - so[l] == r - l {
            out.print_line(r - l);
        } else if sz[r] - sz[l] > 0 && so[r] - so[l] > 0 {
            out.print_line(sz[r] - sz[l] + so[r] - so[l] - 2);
        } else if sz[r] - sz[l] > 0 && (s[l] == b'1' || s[r] == b'1') {
            out.print_line(sz[r] - sz[l] - 1);
        } else if so[r] - so[l] > 0 && (s[l] == b'0' || s[r] == b'0') {
            out.print_line(so[r] - so[l] - 1);
        } else {
            out.print_line(sz[r] - sz[l] + so[r] - so[l]);
        }
    }
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
