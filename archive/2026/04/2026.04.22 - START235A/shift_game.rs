//{"name":"Shift Game","group":"CodeChef - START235A","url":"https://www.codechef.com/START235A/problems/SHFTGAME","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n2 5\n01\n10\n1 3 5 7 9\n3 4\n101\n100\n1 6 7 12\n","output":"0 0 1 1 2\n1 2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
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
    let mut a = input.read_str();
    let mut b = input.read_str();
    let k = input.read_size_vec(q);

    if a.copy_count(a[0]) == n {
        for i in 0..n {
            if a[0] == b[i] && a[0] == b'1' {
                out.print_line_iter(k.iter_map(|x| x.saturating_sub(i)));
                return;
            }
        }
        out.print_line(vec![0; q]);
        return;
    }
    if b.copy_count(b[0]) == n {
        for i in 0..n {
            if b[0] != a[i] && b[0] == b'1' {
                out.print_line_iter(k.iter_map(|x| x.min(i)));
                return;
            }
        }
        out.print_line(vec![0; q]);
        return;
    }
    let mut at_a = 0;
    let mut at_b = 0;
    let mut prefix = Vec::new();
    let mut cur = 0;
    loop {
        if a[at_a % n] == b'0'
            && b[at_b % n] == b'0'
            && at_a > 0
            && at_b > 0
            && a[(at_a - 1) % n] != b'0'
            && b[(at_b - 1) % n] != b'0'
        {
            break;
        }
        if a[at_a % n] != b[at_b % n] {
            at_b += 1;
        } else {
            if a[at_a] == b'1' {
                cur += 1;
            }
            at_a += 1;
        }
        prefix.push(cur);
    }
    a.rotate_left(at_a % n);
    b.rotate_left(at_b % n);
    assert_eq!(a[0], b'0');
    assert_eq!(b[0], b'0');
    fn build(a: &[u8]) -> Vec<usize> {
        let mut cur = a[0];
        let mut q = 0;
        let mut res = Vec::new();
        for &c in a {
            if c != cur {
                res.push(q);
                q = 0;
                cur = c;
            }
            q += 1;
        }
        res.push(q);
        res
    }
    let aa = build(&a);
    let bb = build(&b);
    fn build_sum(aa: &[usize]) -> Vec<usize> {
        let mut cur = 0;
        let mut res = vec![0];
        for i in aa.indices() {
            if i % 2 == 1 {
                cur += aa[i];
            }
            res.push(cur);
        }
        res
    }
    let sums = build_sum(&aa);
    let sa = aa.partial_sums();
    let sb = bb.partial_sums();

    out.print_line(Vec::with_gen(q, |x| {
        let mut k = k[x];
        if k <= prefix.len() {
            return prefix[k - 1];
        }
        k -= prefix.len();
        let mut left = 0;
        let mut right = k;
        while left < right {
            let mid = (left + right + 1) / 2;
            let total = sa[aa.len()]
                .saturating_mul(mid / aa.len())
                .saturating_add(sa[mid % aa.len()])
                .saturating_add(sb[bb.len()].saturating_mul(mid / bb.len()))
                .saturating_add(sb[mid % bb.len()]);
            if total <= k {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        k -= sa[aa.len()] * (left / aa.len())
            + sa[left % aa.len()]
            + sb[bb.len()] * (left / bb.len())
            + sb[left % bb.len()];
        let mut res = prefix[Back(0)] + sums[Back(0)] * (left / aa.len()) + sums[left % aa.len()];
        let mut a_at = left % aa.len();
        let mut b_at = left % bb.len();
        while k > 0 {
            let cur = if a_at % 2 == b_at % 2 {
                aa[a_at]
            } else {
                bb[b_at]
            };
            if a_at % 2 == 1 && b_at % 2 == 1 {
                res += cur.min(k);
            }
            k = k.saturating_sub(cur);
            if a_at % 2 == b_at % 2 {
                a_at = (a_at + 1) % aa.len();
            } else {
                b_at = (b_at + 1) % bb.len();
            };
        }
        res
    }));
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
