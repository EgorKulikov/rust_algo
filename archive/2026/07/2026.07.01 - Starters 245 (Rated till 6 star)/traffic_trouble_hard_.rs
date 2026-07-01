use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut x = input.read_long_vec(n);
    let s = input.read_long_vec(n);

    let mut not_happy = BitSet::new(n);
    for _ in 0..k.min(n) {
        for j in 0..n {
            if j + 1 < n && x[j] + 1 == x[j + 1] {
                not_happy.set(j);
            } else {
                x[j] += s[j];
                if j + 1 < n {
                    let lim = x[j + 1] - 1;
                    x[j].minim(lim);
                }
            }
        }
    }
    let rem = (k - k.min(n)) as i64;
    let mut when = None;
    let mut pos = 0;
    for i in (0..n).rev() {
        if s[i] == 0 {
            not_happy.set(i);
            when = Some(0);
            pos = x[i];
        } else if let Some(w) = when {
            pos -= 1;
            if x[i] == pos && rem > 0 {
                not_happy.set(i);
                when = Some(0);
                continue;
            }
            let t = (pos - x[i]).upper_div(s[i]);
            let nw = (w + 1).max(t);
            if nw < rem {
                not_happy.set(i);
                when = Some(nw);
            } else {
                when = None;
            }
        }
    }
    out.print_line(not_happy.count_ones());
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
