use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_iterator::iterate;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut mem = Memoization3d::new(20, 10, k + 1, |mem, rem, last, fl| -> usize {
        if rem == 0 {
            if fl == 0 {
                1
            } else {
                0
            }
        } else {
            let mut res = 0;
            for i in 0..10 {
                let cur = last.abs_diff(i);
                if cur <= fl {
                    res += mem.call(rem - 1, i, fl - cur);
                }
            }
            res
        }
    });
    let mut ans = 0;
    for (prefix, len, _) in iterate(1, n) {
        let mut cur = prefix / 10;
        let mut last = prefix % 10;
        let mut fl = 0;
        while cur > 0 {
            let next = cur % 10;
            fl += next.abs_diff(last);
            last = next;
            cur /= 10;
        }
        if fl <= k {
            ans += mem.call(len, prefix % 10, k - fl);
        }
    }
    out.print_line(ans);
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
