//{"name":"C2. Key of Like (Hard Version)","group":"Codeforces - Codeforces Round 1012 (Div. 1)","url":"https://codeforces.com/contest/2089/problem/C2","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n3 1 4\n3 2 0\n25 2 5\n4 102 9\n","output":"800000006 800000006 400000003\n500000004 1 500000004\n142857144 166666668 615646263 639455787 234126986 257936510 195918369 502040820 478316330 81264173 190523433 471438023 23809524 0 0 0 0 0 0 0 0 0 0 0 0\n568832210 85779764 969938175 375449967\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::invertible::Invertible;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size();
    let k = input.read_size();

    type Mod = ModInt7;
    let mut ans = vec![Mod::zero(); n];
    let mut cur = Arr2d::new(k + 1, n, Mod::zero());
    cur[(k, 0)] = Mod::one();
    let mut next = Arr2d::new(k + 1, n, Mod::zero());
    let mut aa = vec![Mod::zero(); n * 2 + 1];
    for i in 0..l {
        next.fill(Mod::zero());
        for j in (0..=k).rev() {
            aa.fill(Mod::zero());
            let vars = l - i + j;
            let add = Mod::from_index(vars).inv().unwrap();
            let p1 = l - i - 1;
            let p2 = l - i - 1 + j;
            let (p1, p2) = if p1 + p2 == 0 {
                (Mod::one(), Mod::zero())
            } else {
                (
                    Mod::from_index(p2) / Mod::from_index(p1 + p2),
                    Mod::from_index(p1) / Mod::from_index(p1 + p2),
                )
            };
            let small = vars / n;
            let big = small + 1;
            let big_qty = vars % n;
            let bb = add * Mod::from_index(big) * p1;
            let ss = add * Mod::from_index(small) * p1;
            for l in 0..n {
                aa[l] += cur[(j, l)] * bb;
                aa[l + big_qty] -= cur[(j, l)] * bb;
                aa[l + big_qty] += cur[(j, l)] * ss;
                aa[l + n] -= cur[(j, l)] * ss;
                // for k in 0..big_qty {
                //     let x = cur[(j, l)] * bb;
                //     ans[(l + k) % n] += x;
                //     next[(j, (l + k + 1) % n)] += x;
                // }
                // for k in big_qty..n {
                //     let x = cur[(j, l)] * ss;
                //     ans[(l + k) % n] += x;
                //     next[(j, (l + k + 1) % n)] += x;
                // }
            }
            let small = (l - i) / n;
            let big = small + 1;
            let big_qty = (l - i) % n;
            let bb = add * Mod::from_index(big) * p2;
            let ss = add * Mod::from_index(small) * p2;
            for l in 0..n {
                aa[l] += cur[(j, l)] * bb;
                aa[l + big_qty] -= cur[(j, l)] * bb;
                aa[l + big_qty] += cur[(j, l)] * ss;
                aa[l + n] -= cur[(j, l)] * ss;
                // for k in 0..big_qty {
                //     let x = cur[(j, l)] * bb;
                //     ans[(l + k) % n] += x;
                //     next[(j, (l + k + 1) % n)] += x;
                // }
                // for k in big_qty..n {
                //     let x = cur[(j, l)] * ss;
                //     ans[(l + k) % n] += x;
                //     next[(j, (l + k + 1) % n)] += x;
                // }
            }
            let mut sum = Mod::zero();
            for l in 0..2 * n {
                sum += aa[l];
                ans[l % n] += sum;
                next[(j, (l + 1) % n)] += sum;
            }
            if j > 0 {
                let pp = add * Mod::from_index(j) * p2;
                for k in 0..n {
                    let add = cur[(j, k)] * pp;
                    cur[(j - 1, (k + l - i) % n)] += add;
                }
            }
        }
        swap(&mut cur, &mut next);
    }
    out.print_line(ans);
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
