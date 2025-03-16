//{"name":"L. Uncle Bob and XOR Sum","group":"Universal Cup - The 3rd Universal Cup. Stage 32: Dhaka","url":"https://contest.ucup.ac/contest/1939/problem/10225","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 1\n1\n1\n2 1\n1 2\n4\n2 1\n1 3\n1\n","output":"0\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut a = input.read_unsigned_vec(n);
    let b = input.read_unsigned_vec(k);

    if b.copy_count(0) > 0 {
        out.print_line(0);
        return;
    }

    let mut at = 0;
    for j in 0..31 {
        let mut found = false;
        for k in at..n {
            if a[k].is_set(j) {
                a.swap(at, k);
                found = true;
                break;
            }
        }
        if found {
            for k in at + 1..n {
                if a[k].is_set(j) {
                    a[k] ^= a[at];
                }
            }
            at += 1;
        }
    }
    a.truncate(at);
    type Mod = ModInt7;
    let base = Mod::new(2).power(n);
    let half = Mod::new(2).inv().unwrap();
    let mut ans = Mod::zero();
    for i in usize::iter_all(k) {
        let mut mask = 0;
        for j in 0..k {
            if i.is_set(j) {
                mask |= b[j];
            }
        }
        let mut a = a.clone();
        let mut at = 0;
        let mut cur = base;
        let mut val = 0;
        for j in 0..31 {
            if !mask.is_set(j) {
                continue;
            }
            let mut found = false;
            for k in at..a.len() {
                if a[k].is_set(j) {
                    a.swap(at, k);
                    found = true;
                    break;
                }
            }
            if found {
                if !val.is_set(j) {
                    val ^= a[at];
                }
                for k in at + 1..a.len() {
                    if a[k].is_set(j) {
                        a[k] ^= a[at];
                    }
                }
                cur *= half;
                at += 1;
            }
        }
        if !mask.is_subset(val) {
            cur = Mod::zero();
        }
        if i.count_ones() % 2 == 0 {
            ans += cur;
        } else {
            ans -= cur;
        }
    }
    out.print_line(ans - Mod::one());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
