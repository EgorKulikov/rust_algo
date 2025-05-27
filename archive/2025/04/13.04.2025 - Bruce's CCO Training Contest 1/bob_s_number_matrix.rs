//{"name":"Bob's Number Matrix","group":"DMOJ","url":"https://dmoj.ca/problem/oly20practice100","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 3 2 2\n1 1 2 2 2\n2 2 3 3 1\n4 4 4 4\n1 1 2 3 3\n2 3 4 4 2\n2 1 4 3 2\n1 2 3 4 4\n","output":"28\n76475\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let m = input.read_unsigned();
    let n = input.read_size();
    let restrictions = input.read_vec::<(usize, usize, usize, usize, u32)>(n);

    let mut x = vec![0, h];
    let mut y = vec![0, w];
    for (x1, y1, x2, y2, _) in restrictions.copy_iter() {
        x.push(x1 - 1);
        x.push(x2);
        y.push(y1 - 1);
        y.push(y2);
    }
    x.sort_unstable();
    x.dedup();
    y.sort_unstable();
    y.dedup();

    type Mod = ModInt7;
    let mut max = Arr2d::new(x.len() - 1, y.len() - 1, 0);
    let mut ans = Mod::zero();
    for i in usize::iter_all(n) {
        max.fill(m);
        for j in 0..n {
            let (x1, y1, x2, y2, lim) = restrictions[j];
            for k in max.rows() {
                for l in max.cols() {
                    if x1 - 1 <= x[k] && x[k + 1] <= x2 && y1 - 1 <= y[l] && y[l + 1] <= y2 {
                        max[(k, l)].minim(lim - i.is_set(j) as u32);
                    }
                }
            }
        }
        let mut cur = Mod::one();
        for j in max.rows() {
            for k in max.cols() {
                cur *= Mod::new(max[(j, k)]).power((x[j + 1] - x[j]) * (y[k + 1] - y[k]));
            }
        }
        if i.count_ones() % 2 == 0 {
            ans += cur;
        } else {
            ans -= cur;
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
