use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(4);

    let mut gauge = vec![2000; 4];
    let mut combo = vec![0; 4];
    let mut failed = BitSet::new(4);
    for i in 0..n {
        for j in 0..4 {
            match s[j][i] {
                b'H' => {
                    combo[j] += 1;
                }
                b'C' => {
                    combo[j] = 0;
                }
                b'J' => {
                    combo[j] = 0;
                    gauge[j] -= 50;
                }
                b'A' => {
                    combo[j] = 0;
                    gauge[j] -= 100;
                }
                b'M' => {
                    combo[j] = 0;
                    gauge[j] -= 200;
                }
                _ => unreachable!(),
            }
            if combo[j] == 100 {
                combo[j] = 0;
                gauge[j] -= 30;
                for k in 0..4 {
                    gauge[k] += 30;
                }
            }
        }
        for j in 0..4 {
            if gauge[j] <= 0 {
                failed.set(j);
            }
            gauge[j].minim(2000);
        }
    }
    for j in 0..4 {
        if failed[j] {
            gauge[j] = 0;
        }
    }
    out.print_line(gauge);
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
