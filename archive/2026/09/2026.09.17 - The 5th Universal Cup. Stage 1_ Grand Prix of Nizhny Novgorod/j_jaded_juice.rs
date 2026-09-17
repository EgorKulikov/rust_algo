use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let w = input.read_long();
    let a = input.read_long_vec(n).sorted();

    let sum = a.copy_sum();
    let process = |times: i64| -> (i64, i64) {
        let mut left = (times + 1) * a[n - 1];
        let mut right = w - times * a[n - 1];
        let mut ans = times;
        for i in (0..n - 1).rev() {
            if left >= right {
                return (ans, right - left);
            }
            let can = left;
            right -= can;
            ans += 1;
            left = can + a[i];
        }
        if left >= right {
            return (ans, right - left);
        }
        let full = (right - left) / sum / (n as i64 + 1);
        ans += full * n as i64;
        left += full * sum;
        right -= full * sum * n as i64;
        for i in (0..n).rev() {
            if left >= right {
                return (ans, right - left);
            }
            ans += 1;
            left += a[i];
            right -= sum;
        }
        if left >= right {
            return (ans, right - left);
        }
        unreachable!()
        // let delta = (right - left).upper_div(2 * sum);
        // (ans + delta, right - left - 2 * sum * delta)
    };
    // let v = (1..=w).map(|i| process(i)).collect::<Vec<_>>();
    // let mp = v.min_position();
    // for i in 0..mp {
    //     assert!(v[i] >= v[i + 1]);
    // }
    // for i in mp + 1..v.len() {
    //     assert!(v[i] >= v[i - 1]);
    // }
    /*let mut ans = process(1).0;
    for i in 2.. {
        let cand = process(i).0;
        if cand > ans {
            break;
        }
        ans = cand;
    }
    out.print_line(ans);*/
    let mut left = 1;
    let mut right = (w.upper_div(a[n - 1]) - 1).upper_div(2);
    while right - left > 5 {
        let mid_left = left + (right - left) / 3;
        let mid_right = left + (right - left) * 2 / 3;
        if process(mid_left) <= process(mid_right) {
            right = mid_right;
        } else {
            left = mid_left;
        }
    }
    let mut ans = None;
    for i in left..=right {
        ans.minim(process(i));
    }
    out.print_line(ans.unwrap().0);
    /*
    let mut left = a[n - 1];
    let mut right = w;
    let mut ans = 0;
    loop {
        for i in (0..n).rev() {
            if left >= right {
                out.print_line(ans);
                return;
            }
            let can = left - taken[i];
            right -= can;
            taken[i] += can;
            ans += 1;
            left = taken[i] + a[i];
        }
    }*/
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
