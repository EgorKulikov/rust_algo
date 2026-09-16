use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let q = p.inv();
    let mut set_low = FenwickTree::new(n);
    let mut set_high = FenwickTree::new(n);
    for i in 0..n {
        set_high.add(i, 1i64);
    }
    let left = Vec::with_gen(n, |i| {
        set_high.add(q[i], -1);
        let res = set_low.get(..q[i]) * 4 + set_low.get(q[i] + 1..) + set_high.get(..q[i]);
        set_low.add(q[i], 1);
        res
    });
    let l = left.partial_sums();
    let right = Vec::with_gen_suffix(n, |i, _| {
        set_low.add(q[i], -1);
        let res = set_high.get(q[i] + 1..) * 4 + set_high.get(..q[i]) + set_low.get(q[i] + 1..);
        set_high.add(q[i], 1);
        res
    });
    let r = right.partial_sums();
    let mid = Vec::with_gen(n, |i| {
        set_high.add(q[i], -1);
        let res = set_low.get(q[i] + 1..) + set_high.get(..q[i]);
        set_low.add(q[i], 1);
        res
    });
    let m = mid.partial_sums();
    let mut ans = None;
    let mut start = 0;
    for i in 1..n {
        if q[i] < q[i - 1] {
            ans.minim((l[start] + r[n] - r[i] + m[i] - m[start]) / 2);
            start = i;
        }
    }
    ans.minim((l[start] + m[n] - m[start]) / 2);
    out.print_line(ans);
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
