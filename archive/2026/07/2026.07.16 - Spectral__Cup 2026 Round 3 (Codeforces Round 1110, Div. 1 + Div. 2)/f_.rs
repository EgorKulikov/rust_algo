use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::combinations_arr;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ee = Vec::new();
    let mut total = 0;
    for i in 0..n {
        if a[i] != -1 {
            if a[i] as usize > i {
                out.print_line(0);
                return;
            }
            total += a[i];
            if total as usize > n - 1 {
                out.print_line(0);
                return;
            }
            ee.push(a[i] as usize + 2);
        } else {
            ee.push(1);
        }
    }
    ee.push(1);
    type Mod = ModIntF;
    let c = combinations_arr::<u32, Mod>(n + 1);
    let mut at = 0;
    let mut shift = Arr2d::new(n + 1, n + 1, 0);
    for i in 0..=n {
        for j in i..=n {
            shift[(i, j)] = at;
            at += ee[j];
        }
    }
    let mut ans = vec![0; at];
    for i in 0..=n {
        if ee[i] > 1 {
            ans[shift[(i, i)] + 1] = 1;
        }
        ans[shift[(i, i)]] = 1;
    }
    eprintln!("at = {}", ans.len());
    for i in (0..n).rev() {
        for j in i + 1..=n {
            for k in 0..ee[j] {
                if k == 1 {
                    continue;
                }
                let mut res = Mod::zero();
                let val = |st: usize, end: usize, target: usize| {
                    if target < ee[end] {
                        Mod::from(ans[shift[(st, end)] + target])
                    } else {
                        Mod::zero()
                    }
                };
                for pos in i..j {
                    let left = val(i, pos, (a[pos] + 1) as usize);
                    let right = val(pos + 1, j, k.saturating_sub(1));
                    res += left * right * c[(j - i - 1, pos - i)];
                }
                ans[shift[(i, j)] + k] = res.val();
            }
        }
    }
    out.print_line(ans[shift[(0, n)]]);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
