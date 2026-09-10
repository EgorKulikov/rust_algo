use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let q = p.inv();
    let mut ans = vec![(0, 0); (n + 1) / 2];
    ans[0] = (q[0], q[0]);
    let mut left = q[0];
    let mut right = q[0];
    for i in 1..ans.len() {
        left.minim(q[i]);
        right.maxim(q[i]);
        ans[i] = (left, right);
        if right - left > 2 * i {
            out.print_line(-1);
            return;
        }
    }
    ans[Back(0)] = (0, n - 1);
    for i in ans.indices().rev() {
        let (mut left, mut right) = ans[i];
        left.minim(right.saturating_sub(2 * i));
        if i + 1 < ans.len() {
            left.maxim(ans[i + 1].0);
        }
        right = left + 2 * i;
        ans[i] = (left, right);
    }
    for i in 1..ans.len() {
        assert!(ans[i].0 <= ans[i - 1].0 && ans[i - 1].1 <= ans[i].1);
    }
    out.print_per_line(&ans.inc());
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
