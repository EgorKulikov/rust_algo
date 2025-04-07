//{"name":"G1. Wish Upon a Satellite (Easy Version)","group":"Codeforces - Teza Round 1 (Codeforces Round 1015, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2084/problem/G1","interactive":false,"timeLimit":4000,"tests":[{"input":"8\n2\n1 0\n3\n0 0 0\n3\n0 1 0\n5\n3 2 4 5 1\n7\n0 3 2 5 0 0 0\n10\n1 2 6 5 8 9 0 0 0 0\n5\n0 4 1 0 0\n5\n0 1 5 2 3\n","output":"4\n12\n11\n44\n110\n300\n45\n40\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable5, RecursiveFunction5};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut num_even = 0;
    let mut num_odd = 0;
    let add_even = 0;
    let add_odd = 0;
    let mut even = BitSet::new(n + 1);
    let mut odd = BitSet::new(n + 1);
    for i in 0..n {
        if i % 2 == 0 {
            if a[i] != 0 {
                even.set(a[i]);
            }
            num_even += 1;
        } else {
            if a[i] != 0 {
                odd.set(a[i]);
            }
            num_odd += 1;
        }
    }
    let even_less = Vec::with_gen_prefix(n + 1, |i, even_less| {
        if i == 0 {
            0
        } else {
            even_less[i - 1] + if even[i] { 1 } else { 0 }
        }
    });
    let odd_less = Vec::with_gen_prefix(n + 1, |i, odd_less| {
        if i == 0 {
            0
        } else {
            odd_less[i - 1] + if odd[i] { 1 } else { 0 }
        }
    });
    let mut cache = Arr2d::new(n + 1, n + 1, usize::MAX);
    let mut mem = RecursiveFunction5::new(
        |rec, i: usize, num_even: usize, num_odd: usize, add_even: usize, add_odd: usize| {
            if cache[(i, num_even)] != usize::MAX {
                return cache[(i, num_even)];
            }
            let res = if even[i] {
                i * (num_even + add_even)
                    + rec.call(i - 1, num_even - 1, num_odd, add_even, add_odd + 1)
            } else if odd[i] {
                i * (num_odd + add_odd)
                    + rec.call(i - 1, num_even, num_odd - 1, add_even + 1, add_odd)
            } else {
                let cand_even = if num_even > even_less[i] {
                    i * (num_even + add_even)
                        + rec.call(i - 1, num_even - 1, num_odd, add_even, add_odd + 1)
                } else {
                    0
                };
                let cand_odd = if num_odd > odd_less[i] {
                    i * (num_odd + add_odd)
                        + rec.call(i - 1, num_even, num_odd - 1, add_even + 1, add_odd)
                } else {
                    0
                };
                cand_even.max(cand_odd)
            };
            cache[(i, num_even)] = res;
            res
        },
    );
    // let mut ans = 0;
    // for i in (1..=n).rev() {
    //     if even.last().copied() == Some(i) {
    //         ans += i * (num_even + add_even);
    //         even.pop();
    //         num_even -= 1;
    //         add_odd += 1;
    //     } else if odd.last().copied() == Some(i) {
    //         ans += i * (num_odd + add_odd);
    //         odd.pop();
    //         num_odd -= 1;
    //         add_even += 1;
    //     } else {
    //         let cand_even = if num_even > even.len() {
    //             i * (num_even + add_even)
    //         } else {
    //             0
    //         };
    //         let cand_odd = if num_odd > odd.len() {
    //             i * (num_odd + add_odd)
    //         } else {
    //             0
    //         };
    //         if cand_even > cand_odd {
    //             ans += cand_even;
    //             num_even -= 1;
    //             add_odd += 1;
    //         } else {
    //             ans += cand_odd;
    //             num_odd -= 1;
    //             add_even += 1;
    //         }
    //     }
    // }
    out.print_line(mem.call(n, num_even, num_odd, add_even, add_odd));
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
