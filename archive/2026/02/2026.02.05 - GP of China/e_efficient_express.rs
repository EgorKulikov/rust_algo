//{"name":"E. Efficient Express","group":"Universal Cup - GP of China","url":"https://contest.ucup.ac/contest/3295/problem/16332","interactive":false,"timeLimit":1000,"tests":[{"input":"4 3 4\n2 3 3 2\n1 2\n2 2\n2 3\n","output":"48\n"},{"input":"7 3 5\n2 4 3 2 4 2 3\n2 2\n4 3\n5 2\n","output":"80\n"},{"input":"4 2 10\n3 7 2 5\n1 4\n3 3\n","output":"100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec();
    let px = input.read_size_pair_vec(m).dec().sorted_by_key(|&(_, x)| x);

    let mut left = vec![None; k];
    let mut right = vec![None; k];
    for i in 0..n {
        left[a[i]].minim(i);
        right[a[i]].maxim(i);
    }
    for i in 0..k {
        if left[i].is_none() {
            continue;
        }
        if left[i] == right[i] {
            left[i] = None;
            right[i] = None;
            continue;
        }
        for j in 0..m {
            let (p, x) = px[j];
            if right[i].unwrap() <= p && x <= i {
                left[i] = None;
                right[i] = None;
                break;
            }
        }
    }

    type Mod = ModIntF;
    let mut ans = Mod::from(0);
    for i in 0..k {
        let mut qty = vec![0; n];
        let mut qty_same = vec![0; n];
        let mut less = 0;
        for (p, x) in px.copy_iter() {
            if x <= i {
                if x < i {
                    less += 1;
                } else {
                    qty_same[p] += 1;
                }
            } else {
                qty[p] += 1;
            }
        }
        // let q = Mod::from(k - i).power(less) * Mod::from(k).power(same)
        //     - Mod::from(k - i - 1).power(less + same);
        let mut need = vec![k; n];
        for j in 0..i {
            if let Some(l) = left[j] {
                need[l].minim(j);
            }
        }
        let mut cur0 = vec![Mod::zero(); k + 1];
        let mut cur1 = vec![Mod::zero(); k + 1];
        cur0[k] = Mod::from(1);
        for j in 0..n {
            for x in need[j] + 1..=k {
                cur0[x] = Mod::from(0);
                cur1[x] = Mod::from(0);
            }
            for _ in 0..qty[j] {
                let mut sum1 = Mod::zero();
                let mut sum0 = Mod::zero();
                for x in (0..=k).rev() {
                    let c = cur1[x];
                    cur1[x] = sum1 + c * (k - x);
                    sum1 += c;
                    let c = cur0[x];
                    cur0[x] = sum0 + c * (k - x);
                    sum0 += c;
                }
            }
            for _ in 0..qty_same[j] {
                let mut sum1 = Mod::zero();
                let mut sum0 = Mod::zero();
                for x in (0..=k).rev() {
                    let c = cur1[x];
                    cur1[x] = sum1 + c * (k - x);
                    sum1 += c;
                    let c = cur0[x];
                    if x <= i {
                        cur1[x] += sum0 + c * (i + 1 - x);
                        cur0[x] = c * (k - (i + 1));
                    } else {
                        cur0[x] = sum0 + c * (k - x);
                    }
                    sum0 += c;
                }
            }
        }
        let mut sum0 = Mod::zero();
        let mut sum1 = Mod::zero();
        for x in 0..=k {
            sum0 += cur0[x];
            sum1 += cur1[x];
        }
        ans += sum0 * (Mod::from(k - i).power(less) - Mod::from(k - i - 1).power(less))
            + sum1 * Mod::from(k - i).power(less);
    }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
