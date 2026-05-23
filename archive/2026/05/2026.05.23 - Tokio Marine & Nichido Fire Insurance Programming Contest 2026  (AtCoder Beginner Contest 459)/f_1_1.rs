use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut val = vec![0; n + 1];
    val[n] = i64::MAX;
    let mut ends = BTreeSet::new();
    ends.insert(n);
    let mut ans = 0;
    for i in (0..n).rev() {
        if a[i] < val[i + 1] {
            val[i] = a[i];
            ends.insert(i);
            continue;
        }
        ends.remove(&(i + 1));
        let mut need = a[i] - val[i + 1] + 1;
        val[i] = a[i];
        loop {
            let pos = *ends.first().unwrap();
            let delta = val[pos] - val[pos - 1] - 1;
            if need <= ((pos - i) as i64).saturating_mul(delta) {
                let full = need / (pos - i) as i64;
                val[i] -= full * (pos - i - 1) as i64;
                val[i + 1] += full;
                if i + 2 != pos {
                    val[pos - 1] += full;
                }
                ans += full * ((pos - i - 1) * (pos - i) / 2) as i64;
                let rem = need - full * (pos - i) as i64;
                if rem > 0 {
                    val[i] -= rem;
                    val[pos - 1] += 1;
                    if rem != 1 {
                        val[pos - rem as usize] += 1;
                    }
                    ans += (2 * (pos - i) as i64 - rem - 1) * rem / 2;
                    ends.insert(pos - rem as usize);
                    val[pos - rem as usize - 1] = val[pos - rem as usize] - 2;
                }
                ends.insert(i);
                break;
            }
            val[i] -= delta * (pos - i - 1) as i64;
            val[i + 1] += delta;
            if i + 2 != pos {
                val[pos - 1] += delta;
            }
            ans += delta * ((pos - i - 1) * (pos - i) / 2) as i64;
            need -= (pos - i) as i64 * delta;
            ends.pop_first();
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
