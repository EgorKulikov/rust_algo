use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::{BitIter, BitOps};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let lrv = input.read_vec::<(i32, i32, i32)>(n);

    let mut good = BitSet::new(1 << n);
    let mut profit = vec![0; 1 << n];
    'outer: for i in usize::iter_all(n) {
        for j in 0..n {
            if !i.is_set(j) {
                continue;
            }
            let (l1, r1, _) = lrv[j];
            for k in 0..j {
                if !i.is_set(k) {
                    continue;
                }
                let (l2, r2, _) = lrv[k];
                if l1 < r2 && l2 < r1 {
                    continue 'outer;
                }
            }
        }
        for j in 0..n {
            if i.is_set(j) {
                profit[i] += lrv[j].2;
            }
        }
        good.set(i);
    }
    let mut ans = None;
    for i in usize::iter_all(n) {
        if i.count_ones() != (n - k) as u32 {
            continue;
        }
        let mut cur = 0;
        for j in BitIter::new(i) {
            if good[j] {
                cur.maxim(profit[j]);
            }
        }
        ans.minim(cur);
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
