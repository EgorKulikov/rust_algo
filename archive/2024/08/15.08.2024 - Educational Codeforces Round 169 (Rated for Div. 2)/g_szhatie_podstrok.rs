//{"name":"G. Сжатие подстрок","group":"Codeforces - Educational Codeforces Round 169 (Rated for Div. 2)","url":"https://codeforces.com/contest/2004/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4\n5999\n","output":"14\n"},{"input":"10 3\n1111111111\n","output":"2 2 2 2 2 2 2 2\n"},{"input":"11 4\n49998641312\n","output":"12 18 17 15 12 7 7 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSzhatiePodstrok"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;
use algo_lib::{add_assign, mul_assign};
use std::ops::{Add, AddAssign, Mul, MulAssign};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input
        .read_str()
        .into_iter()
        .map(|c| (c - b'0') as usize)
        .collect_vec();

    #[derive(Eq, PartialEq, Copy, Clone)]
    struct Number(i64);

    impl Add for Number {
        type Output = Number;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0.min(rhs.0))
        }
    }

    add_assign!(Number);

    impl Zero for Number {
        fn zero() -> Self {
            Self(i64::MAX / 2)
        }
    }

    impl Mul for Number {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }

    mul_assign!(Number);

    impl One for Number {
        fn one() -> Self {
            Self(0)
        }
    }

    const SIDE: usize = 10;
    let matrices = (0..=9)
        .map(|i| {
            let mut matrix = Matrix::zero(SIDE, SIDE);
            for j in 0..9 {
                matrix[(j, j)] = Number((j + 1) as i64);
                matrix[(j, 9)] = Number((j + 1) as i64);
                if j + 1 == i {
                    matrix[(9, j)] = Number(0);
                }
            }
            matrix
        })
        .collect::<Vec<_>>();

    let mut ans = Vec::with_capacity(n - k + 1);
    let mut ltr = Vec::with_capacity(n);
    let mut rtl = Vec::with_capacity(n - n % k);
    let ident = Matrix::ident(SIDE);
    for i in 0..n {
        let cur = if i % k == 0 { &ident } else { &ltr[i - 1] };
        let res = cur.mult(&matrices[s[i]]);
        ltr.push(res);
    }
    for i in (0..n - n % k).rev() {
        let cur = if (i + 1) % k == 0 {
            &ident
        } else {
            &rtl[rtl.len() - 1]
        };
        let res = matrices[s[i]].mult(cur);
        rtl.push(res);
    }
    rtl.reverse();
    for i in 0..=n - k {
        if i % k == 0 {
            ans.push(ltr[i + k - 1][(SIDE - 1, SIDE - 1)].0);
        } else {
            let res = rtl[i].mult(&ltr[i + k - 1]);
            ans.push(res[(SIDE - 1, SIDE - 1)].0);
        }
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
