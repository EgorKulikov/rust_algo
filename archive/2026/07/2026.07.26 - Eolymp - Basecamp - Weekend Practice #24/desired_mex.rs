use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size() - 1;
    let r = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    if k > r - l {
        out.print_line(0);
        return;
    }
    let mut max_start = r - 1;
    let mut min_finish = 0;
    let mut bad = 0;
    for i in l..r {
        if a[i] == k {
            max_start.minim(i);
            min_finish.maxim(i + 1);
            bad += 1;
        }
    }
    let mut seen = BitSet::new(k);
    let mut needed = FenwickTree::new(n);
    let mut x = vec![n; n];
    let mut y = vec![0; n];
    for i in l..r {
        if a[i] < k {
            seen.set(a[i]);
            if x[a[i]] == n {
                x[a[i]] = i;
            }
            y[a[i]] = i;
        }
    }
    for i in 0..k {
        if seen[i] {
            needed.add(y[i], 1);
        }
    }
    let need = k - seen.count_ones();
    if bad == 0 && need == 0 {
        out.print_line(n * (n + 1) / 2);
        return;
    }
    let mut first = vec![n; k];
    let mut next = vec![0; n];
    for i in (0..n).rev() {
        if a[i] < k {
            next[i] = first[a[i]];
            first[a[i]] = i;
        }
    }
    let mut ans = 0;
    let mut min_end = 0;
    for i in 0..k {
        if !seen[i] {
            min_end.maxim(first[i] + 1);
        }
    }
    let not_bad = Vec::with_gen(n, |i| if a[i] != k { 1 } else { 0 });
    let s = not_bad.partial_sums();
    for i in 0..=max_start {
        let mut end = (i + 1).max(l + need).max(min_end).max(min_finish);
        let outside = l.saturating_sub(i) + end.saturating_sub(r);
        if outside < bad {
            end = end.max(r) + bad - outside;
        }
        let mut left = end;
        let mut right = n + 1;
        while left < right {
            let mid = (left + right) / 2;
            let len = r.min(mid) - i.max(l);
            let free = len - needed.get(i..mid) as usize;
            if s[mid] - s[i] >= len && free >= need {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        end = left;
        if end <= n {
            ans += n + 1 - end;
        }
        if a[i] < k && !seen[a[i]] {
            min_end.maxim(next[i] + 1);
        }
        if a[i] < k && i == x[a[i]] {
            needed.add(y[a[i]], -1);
        }
    }
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
