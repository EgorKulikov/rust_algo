use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_long();
    let mut h = input.read_long_vec(n);

    let mut cd = vec![0; n];
    let mut last = vec![0; n];
    let mut queue = BTreeSet::new();
    let mut cur_time = vec![0; n];
    for i in 0..n {
        if h[i] <= 0 {
            queue.insert((0, i));
        }
    }
    let mut at = 0;
    let mut ans = 0;
    while let Some((time, id)) = queue.pop_first() {
        if time > at + 1 {
            break;
        }
        h[id] = 0;
        ans += 1;
        let mut cand = Vec::new();
        if id > 0 {
            cand.push(id - 1);
        }
        if id + 1 < n {
            cand.push(id + 1);
        }
        at = time;
        for c in cand {
            if h[c] <= 0 {
                continue;
            }
            h[c] -= cd[c] * (time - last[c]);
            queue.remove(&(cur_time[c], c));
            cd[c] += d;
            cur_time[c] = time + h[c].upper_div(cd[c]);
            queue.insert((cur_time[c], c));
            last[c] = time;
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
