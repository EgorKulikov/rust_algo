//{"name":"E. Blossom","group":"Codeforces - Teza Round 1 (Codeforces Round 1015, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2084/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n2\n0 -1\n2\n-1 -1\n3\n2 0 1\n3\n-1 2 -1\n5\n-1 0 -1 2 -1\n","output":"3\n6\n7\n10\n104\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut left_min = n;
    let mut unknown = BitSet::new(n);
    unknown.fill(true);
    let right_min = Vec::with_gen_back(n + 1, |i, right_min| -> usize {
        if i == n {
            n
        } else if a[i] == -1 {
            right_min[i + 1]
        } else {
            unknown.unset(a[i] as usize);
            right_min[i + 1].min(a[i] as usize)
        }
    });
    let unknown = unknown.iter().collect::<Vec<_>>();
    let num_wildcards = unknown.len();
    let mut qty = Arr2d::new(num_wildcards + 1, n + 1, 0);
    for i in 0..n {
        let mut wildcards = 0;
        for j in i..n {
            if a[j] == -1 {
                wildcards += 1;
            }
            qty[(wildcards, left_min.min(right_min[j + 1]))] += 1;
        }
        if a[i] != -1 {
            left_min.minim(a[i] as usize);
        }
    }
    type Mod = ModInt7;
    let mut ans = Mod::zero();
    let c = Combinations::<Mod>::new(n + 1);
    for i in 0..=num_wildcards {
        let mut at = n + 1;
        let mut total = Mod::zero();
        for j in (0..=i).rev() {
            let from = if j == 0 { 0 } else { unknown[j - 1] };
            let mut cur = Mod::zero();
            for k in from..at {
                cur += Mod::new_signed(qty[(i, k)])
                    * Mod::from_index(k.min(*unknown.get(i).unwrap_or(&n)));
            }
            ans += cur * c.c(i, j) * c.fact(j) * c.fact(num_wildcards - j);
            if i != num_wildcards {
                // ans += cur * c.fact(num_wildcards);
                // } else {
                ans += total
                    * Mod::from_index(at)
                    * (c.c(i, j)
                        * Mod::from_index(num_wildcards - i)
                        * c.fact(j)
                        * c.fact(num_wildcards - j - 1));
            }
            for k in from..at {
                total += Mod::new_signed(qty[(i, k)]);
            }
            at = from;
        }
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
