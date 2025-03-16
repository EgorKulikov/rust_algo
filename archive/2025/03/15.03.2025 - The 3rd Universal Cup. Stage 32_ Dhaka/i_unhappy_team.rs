//{"name":"I. Unhappy Team","group":"Universal Cup - The 3rd Universal Cup. Stage 32: Dhaka","url":"https://contest.ucup.ac/contest/1939/problem/10222","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 2\nXSW\nSXW\nWSX\n4 3\nXSWS\nWXSW\nSSXS\nSWWX\n1 1\nX\n","output":"499122178\n499122179\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_utils::factorial;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    eprintln!("Processing test case {}", test_case);
    let n = input.read_size();
    let k = input.read_size();
    let t = input.read_char_table(n, n);

    type Mod = ModIntF;
    let bad = Arr2d::with_gen(1 << n, n, |mask, i| {
        let mut res = 0;
        for j in 0..n {
            if !mask.is_set(j) && t[(i, j)] == b'W' {
                res += 1;
            }
        }
        res
    });
    let mut res = Arr3d::new(1 << n, n + 1, n + 1, 0i64);
    let mut ways = Arr3d::new(1 << n, n + 1, n + 1, 0i64);
    let mut ans = Mod::zero();
    for w in 0..=n - k {
        res.fill(0);
        ways.fill(0);
        ways[(0, 0, 0)] = 1;
        for mask in usize::iter_all(n).skip(1) {
            for rem in 0..=k {
                for rem_border in 0..=rem {
                    let mut ww = 0;
                    let mut rr = 0;
                    for i in 0..n {
                        if mask.is_set(i) {
                            let cur = bad[(mask, i)];
                            if cur > w || cur == w && rem_border > 0 {
                                if rem > 0 {
                                    let aa = mask.without_bit(i);
                                    let bb = rem - 1;
                                    let cc = rem_border - ((cur == w) as usize);
                                    let call_ways = ways[(aa, bb, cc)];
                                    let call_res = res[(aa, bb, cc)];
                                    ww += call_ways;
                                    rr += call_res + call_ways * cur as i64;
                                }
                            } else {
                                let aa = mask.without_bit(i);
                                let bb = rem;
                                let cc = rem_border;
                                let call_ways = ways[(aa, bb, cc)];
                                let call_res = res[(aa, bb, cc)];
                                ww += call_ways;
                                rr += call_res;
                            }
                        }
                    }
                    ways[(mask, rem, rem_border)] = ww;
                    res[(mask, rem, rem_border)] = rr;
                }
            }
        }
        for i in 1..=k {
            ans += Mod::new_wide(res[(usize::all_bits(n), k, i)]);
        }
    }
    ans /= factorial(n);
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
