use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut k = input.read_size();

    let mut a = vec![0; n];
    let mut p = vec![0; n];
    let mut rem = (0..n).collect::<FxHashSet<_>>();
    a[n - 1] = n;
    k ^= n;
    let mut pos = n;
    if n >= 2 && k != 0 {
        let mut val = 0;
        let mut enabled = false;
        for i in (0..30).rev() {
            if k.is_set(i) {
                if enabled || (n - 1).is_set(i) {
                    val.set_bit(i);
                }
            } else {
                if (n - 1).is_set(i) {
                    enabled = true;
                }
            }
        }
        a[n - 2] = val;
        k ^= a[n - 2];
        p[n - 1] = a[n - 2];
        rem.remove(&a[n - 2]);
        pos -= 1;
    }
    if n >= 3 && k != 0 {
        if n - 2 >= k {
            a[n - 3] = k;
            pos -= 1;
            p[n - 2] = k;
            rem.remove(&k);
            k = 0;
        }
    }
    if k != 0 {
        out.print_line(false);
        return;
    }
    p[pos - 1] = 0;
    rem.remove(&0);
    for (i, j) in rem.into_iter().enumerate() {
        p[i] = j;
    }
    out.print_line(true);
    out.print_line(p);
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
