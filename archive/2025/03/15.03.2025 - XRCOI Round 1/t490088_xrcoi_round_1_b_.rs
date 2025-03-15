//{"name":"T490088 [XRCOI Round 1] B. 稻花香里说丰年","group":"Luogu","url":"https://www.luogu.com.cn/problem/T490088?contestId=221170","interactive":false,"timeLimit":2500,"tests":[{"input":"3 0\n1 0\n1 1\n0 0\n","output":"1\n"},{"input":"5 0\n0 1\n1 0\n0 1\n1 1\n1 1\n","output":"70\n"},{"input":"4 0\n0 1\n1 0\n0 1\n1 0\n","output":"52\n"},{"input":"5 1\n1 1 0 1 1\n1 1 1 1 1\n","output":"22\n"},{"input":"10000 1\n1 1 0 1 1\n1 1 1 1 1\n","output":"559297012\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let op = input.read_int();
    let mut ab = if op == 0 {
        input.read_vec::<(i8, i8)>(n)
    } else {
        let mut build_array = || {
            let x = input.read_u64();
            let y = input.read_u64();
            let z = input.read_u64();
            let mut prev = input.read_u64();
            let mut cur = input.read_u64();
            let len = n.upper_div(64);
            let mut res = Vec::with_capacity(len);
            for _ in 0..len {
                let next = prev
                    .overflowing_mul(x)
                    .0
                    .overflowing_add(cur.overflowing_mul(y).0)
                    .0
                    .overflowing_add(z)
                    .0;
                res.push(next);
                prev = cur;
                cur = next;
            }
            res
        };
        let f = build_array();
        let g = build_array();
        let mut ab = Vec::with_capacity(n);
        for i in 0..n {
            ab.push((
                f[i / 64].is_set(i % 64) as i8,
                g[i / 64].is_set(i % 64) as i8,
            ));
        }
        ab
    };

    for i in 0..n {
        ab[i].1 = 1 - ab[i].1;
    }

    type Mod = ModInt7;
    let mut ans = Mod::zero();
    for _ in 0..2 {
        let mut dp = Arr2d::new(3, 2, Mod::zero());
        dp[(0, 0)] = Mod::new(2).inv().unwrap();
        let mut next = Arr2d::new(3, 2, Mod::zero());
        for i in 0..n {
            let (a, b) = ab[i];
            next.fill(Mod::zero());
            for j in 0..3 {
                for k in 0..2 {
                    let mult = if (j, k) == (0, 0) || (j, k) == (2, 1) {
                        Mod::new(2)
                    } else {
                        Mod::one()
                    };
                    next[(j, k)] += dp[(j, k)] * mult;
                    if j == 0 && a == 0 {
                        if k == 0 && a == b {
                            next[(1, 1)] += dp[(j, k)] * mult;
                        }
                        next[(1, k)] += dp[(j, k)] * mult;
                    }
                    if j == 1 && a == 1 {
                        if k == 0 && a == b {
                            next[(2, 1)] += dp[(j, k)] * mult;
                        }
                        next[(2, k)] += dp[(j, k)] * mult;
                    }
                    if k == 0 && a == b {
                        next[(j, 1)] += dp[(j, k)] * mult;
                    }
                }
            }
            std::mem::swap(&mut dp, &mut next);
        }
        ans += dp[(2, 1)];
        let (a, b) = ab.detuple();
        ab = b.iter_zip(a).collect();
    }
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
