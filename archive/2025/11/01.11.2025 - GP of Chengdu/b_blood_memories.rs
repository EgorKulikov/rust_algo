//{"name":"B. Blood Memories","group":"Universal Cup - GP of Chengdu","url":"https://contest.ucup.ac/contest/2567/problem/14707","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 7 1 5\n59 3\n13 2\n81 4\n5 14 2 9\n66 8\n20 2\n25 4\n39 6\n57 7\n4 13 7 16\n18 2\n13 5\n33 4\n7 1\n","output":"490\n939\n741\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::add;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::ops::{Add, AddAssign, Mul};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let r = input.read_size();
    let ac = input.read_vec::<(i64, usize)>(n);

    #[derive(Copy, Clone)]
    struct AddMax(i64);
    impl AddAssign for AddMax {
        fn add_assign(&mut self, rhs: Self) {
            self.0.maxim(rhs.0);
        }
    }
    add!(AddMax);
    impl Mul for AddMax {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            AddMax(self.0 + rhs.0)
        }
    }
    impl Zero for AddMax {
        fn zero() -> Self {
            AddMax(i64::MIN / 2)
        }
    }
    impl One for AddMax {
        fn one() -> Self {
            AddMax(0)
        }
    }
    let mut mat = Matrix::zero(1 << n, 1 << n);
    for from in usize::iter_all(n) {
        for to in usize::iter_all(n) {
            let mut sum = 0;
            let mut dam = 0;
            for i in 0..n {
                if to.is_set(i) {
                    sum += ac[i].1;
                    dam += ac[i].0;
                    if from.is_set(i) {
                        sum += k;
                    }
                }
            }
            if sum <= m {
                mat[(from, to)] = AddMax(dam);
            }
        }
    }
    let ans = mat.power(r);
    let mut res = 0;
    for i in usize::iter_all(n) {
        res.maxim(ans[(0, i)].0);
    }
    out.print_line(res);
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
