use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
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

    let qty = Arr2d::with_gen(2, 2, |i, j| {
        let mut x = Vec::with_capacity(n - 1);
        for (a, b) in s.consecutive_iter_copy() {
            if a == i as u8 + b'0' && b == j as u8 + b'0' {
                x.push(1);
            } else {
                x.push(0);
            }
        }
        x.partial_sums()
    });

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size() - 1;

        let q = Arr2d::with_gen(2, 2, |i, j| {
            qty[(i, j)][r] - qty[(i, j)][l] + if s[r] == i as u8 + b'0' && s[l] == j as u8 + b'0' { 1 } else { 0 }
        });
        assert_eq!(q[(0, 1)], q[(1, 0)]);
        let mut q00 = q[(0, 0)];
        let mut q01 = q[(0, 1)];
        let mut q11 = q[(1, 1)];
        let mut ans = 0;
        if q00 > q01 && q11 > q01 {
            if q00 > q11 {
                let delta = (q00 - q11).min(q11 - q01);
                ans += delta;
                q00 -= delta;
                q01 += delta;
            } else {
                let delta = (q11 - q00).min(q00 - q01);
                ans += delta;
                q11 -= delta;
                q01 += delta;
            }
        }
        if q00 > q01 && q11 <= q01 {
            let delta = (q00 - q01) / 2;
            ans += delta;
            q00 -= delta;
            q01 += delta;
        }
        if q11 > q01 && q00 <= q01 {
            let delta = (q11 - q01) / 2;
            ans += delta;
            q11 -= delta;
            q01 += delta;
        }
        if q00 == q11 && q00 > q01 {
            let delta = (q00 - q01) / 3;
            ans += delta * 2;
            q00 -= delta;
            q11 -= delta;
            q01 += delta * 2;
        }
        let max = q00.max(q11).max(q01);
        ans += max - q00;
        ans += max - q11;
        ans += 2 * (max - q01);
        out.print_line(ans);
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
