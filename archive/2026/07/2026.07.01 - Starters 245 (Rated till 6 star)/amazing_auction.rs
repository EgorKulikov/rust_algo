use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);
    let c = input.read_long_vec(n);

    let mut order = Vec::new();
    let mut val = Vec::new();
    let mut delta = Vec::new();
    let mut ans = 0;
    let mut sum_val = 0;
    let mut sum_delta = 0;
    let mut poi = a
        .copy_enumerate()
        .map(|(i, v)| (v, i))
        .collect::<BTreeSet<_>>();
    let mut at = 0;
    let mut pos = vec![None; n];
    let mut inside = 0;
    let max = a.copy_max();
    while let Some((x, id)) = poi.pop_first() {
        let d = x - at;
        sum_val += sum_delta * d;
        ans.maxim(k as i64 * x - sum_val);
        at = x;
        if x >= max {
            break;
        }

        if let Some(p) = pos[id] {
            if p == 0 {
                continue;
            }
            let v1 = val[p - 1] + x * delta[p - 1];
            let v2 = val[p] + x * delta[p];
            if v1 + delta[p - 1] <= v2 + delta[p] {
                continue;
            }
            val.swap(p - 1, p);
            delta.swap(p - 1, p);
            order.swap(p - 1, p);
            pos[order[p - 1]] = Some(p - 1);
            pos[order[p]] = Some(p);
            if p + 1 < order.len() && delta[p] > delta[p + 1] {
                poi.insert((
                    (val[p + 1] - val[p] + delta[p + 1] * x - delta[p] * x)
                        / (delta[p] - delta[p + 1])
                        + x,
                    order[p + 1],
                ));
            }
            if p > 1 && delta[p - 2] > delta[p - 1] {
                poi.insert((
                    (val[p - 1] - val[p - 2] + delta[p - 1] * x - delta[p - 2] * x)
                        / (delta[p - 2] - delta[p - 1])
                        + x,
                    order[p - 1],
                ));
            }
            if p == inside {
                sum_val -= val[p] + x * delta[p];
                sum_val += val[p - 1] + x * delta[p - 1];
                sum_delta -= delta[p];
                sum_delta += delta[p - 1];
            }
        } else {
            order.push(id);
            val.push(-c[id] * x);
            delta.push(c[id]);
            order.rotate_right(1);
            val.rotate_right(1);
            delta.rotate_right(1);
            for i in order.indices() {
                pos[order[i]] = Some(i);
            }
            if order.len() > 1 && delta[0] > delta[1] {
                poi.insert((
                    (val[1] - val[0] + delta[1] * x - delta[0] * x) / (delta[0] - delta[1]) + x,
                    order[1],
                ));
            }
            let rem = n - order.len();
            if rem <= k {
                inside = k - rem + 1;
                sum_val = 0;
                sum_delta = 0;
                for i in 0..inside {
                    sum_val += val[i] + delta[i] * x;
                    sum_delta += delta[i];
                }
            }
        }
    }
    out.print_line(ans);
    /*let mut left = 0;
    let mut right = a.copy_max();
    while left < right {
        let mid = (left + right + 1) / 2;

        let mut cur = Vec::with_capacity(n);
        let mut prev = Vec::with_capacity(n);
        for i in 0..n {
            if a[i] >= mid {
                cur.push(0);
            } else {
                cur.push((mid - a[i]) * c[i]);
            }
            if a[i] >= mid - 1 {
                prev.push(0);
            } else {
                prev.push((mid - 1 - a[i]) * c[i]);
            }
        }
        cur.sort();
        prev.sort();
        let cur_val = cur.copy_take(k + 1).sum::<i64>();
        let prev_val = prev.copy_take(k + 1).sum::<i64>();
        if cur_val - prev_val < k as i64 {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    let mut ans = k as i64 * left;
    let mut cur = Vec::with_capacity(n);
    for i in 0..n {
        if a[i] >= left {
            cur.push(0);
        } else {
            cur.push((left - a[i]) * c[i]);
        }
    }
    cur.sort();
    let cur_val = cur.copy_take(k + 1).sum::<i64>();
    ans -= cur_val;
    out.print_line(ans);*/
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
