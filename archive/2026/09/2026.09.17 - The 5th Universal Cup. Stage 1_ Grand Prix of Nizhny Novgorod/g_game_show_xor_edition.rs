use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::fwht::FWHT;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut v = vec![Vec::new(); k];
    let mut ans = Vec::new();
    for i in 0..n {
        if a[i] > 0 {
            v[a[i].highest_bit()].push(a[i]);
        } else {
            ans.push(0);
        }
    }
    type Mod = ModIntF;
    let mut can = vec![vec![true]];
    for i in 0..k {
        let mut a = vec![Mod::zero(); 1 << (i + 1)];
        for j in can[i].indices() {
            if can[i][j] {
                a[j] = Mod::one();
            }
        }
        let mut b = vec![Mod::zero(); 1 << (i + 1)];
        for j in v[i].copy_iter() {
            b[j] = Mod::one();
        }
        if v[i].is_empty() {
            b[0] = Mod::one();
        }
        a.fwht(false);
        b.fwht(false);
        let mut c = a.copy_zip(&b).map(|x| x.0 * x.1).collect::<Vec<_>>();
        c.fwht(true);
        can.push(c.copy_map(|x| x != Mod::zero()).collect());
    }
    assert!(can[k][1 << (k - 1)]);
    let mut at = 1 << (k - 1);
    for i in (0..k).rev() {
        let mut last = None;
        if v[i].is_empty() {
            continue;
        }
        for j in v[i].copy_iter() {
            if last.is_some() {
                ans.push(j);
                continue;
            }
            if can[i][j ^ at] {
                at ^= j;
                last = Some(j);
            } else {
                ans.push(j);
            }
        }
        ans.push(last.unwrap());
    }
    ans.reverse();
    out.print_line(ans);
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
