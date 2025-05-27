//{"name":"Bob's Dominos","group":"DMOJ","url":"https://dmoj.ca/problem/oly21practice32","interactive":false,"timeLimit":1000,"tests":[{"input":"2 4\n0 0 0 0\n0 0 0 1\n","output":"14 13 10 22\n15 11 17 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_table(n, m);

    type Mod = ModInt7;
    let mut res_up = Arr3d::new(n, m + 1, 1 << m, Mod::zero());
    for r in 0..n {
        for mask in usize::iter_all(m) {
            if r == 0 {
                if mask == 0 {
                    res_up[(r, m, mask)] = Mod::one();
                } else {
                    res_up[(r, m, mask)] = Mod::zero();
                }
            } else {
                res_up[(r, m, mask)] = res_up[(r - 1, 0, mask)];
            }
        }
        for c in (0..m).rev() {
            for mask in usize::iter_all(m) {
                if mask.is_set(c) || a[(r, c)] == 1 {
                    if mask.is_set(c) && a[(r, c)] == 1 {
                        res_up[(r, c, mask)] = Mod::zero();
                    } else {
                        res_up[(r, c, mask)] = res_up[(r, c + 1, mask.without_bit(c))];
                    }
                } else {
                    res_up[(r, c, mask)] =
                        res_up[(r, c + 1, mask)] + res_up[(r, c + 1, mask.with_bit(c))];
                    if c + 1 != m && !mask.is_set(c + 1) {
                        let add = res_up[(r, c + 1, mask.with_bit(c + 1))];
                        res_up[(r, c, mask)] += add;
                    }
                }
            }
        }
    }
    let mut res_down = Arr3d::new(n, m + 1, 1 << m, Mod::zero());
    for r in (0..n).rev() {
        for mask in usize::iter_all(m) {
            if r == n - 1 {
                if mask == 0 {
                    res_down[(r, m, mask)] = Mod::one();
                } else {
                    res_down[(r, m, mask)] = Mod::zero();
                }
            } else {
                res_down[(r, m, mask)] = res_down[(r + 1, 0, mask)];
            }
        }
        for c in (0..m).rev() {
            for mask in usize::iter_all(m) {
                if mask.is_set(c) || a[(r, c)] == 1 {
                    if mask.is_set(c) && a[(r, c)] == 1 {
                        res_down[(r, c, mask)] = Mod::zero();
                    } else {
                        res_down[(r, c, mask)] = res_down[(r, c + 1, mask.without_bit(c))];
                    }
                } else {
                    res_down[(r, c, mask)] =
                        res_down[(r, c + 1, mask)] + res_down[(r, c + 1, mask.with_bit(c))];
                    if c + 1 != m && !mask.is_set(c + 1) {
                        let add = res_down[(r, c + 1, mask.with_bit(c + 1))];
                        res_down[(r, c, mask)] += add;
                    }
                }
            }
        }
    }
    let ans = Arr2d::with_gen(n, m, |r, c| {
        if a[(r, c)] == 1 {
            Mod::zero()
        } else {
            let mut res = Mod::zero();
            for mask in usize::iter_all(m) {
                if mask.is_set(c) {
                    continue;
                }
                res += res_up[(r, m, mask)] * res_down[(r, 0, mask.with_bit(c))];
            }
            res
        }
    });
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
