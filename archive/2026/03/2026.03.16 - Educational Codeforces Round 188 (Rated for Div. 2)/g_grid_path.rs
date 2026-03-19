//{"name":"G. Grid Path","group":"Codeforces - Educational Codeforces Round 188 (Rated for Div. 2)","url":"https://codeforces.com/contest/2204/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"2 2 100\n","output":"7\n"},{"input":"1 5 777\n","output":"5\n"},{"input":"5 3 998244353\n","output":"1695\n"},{"input":"100000000 150 1000000000\n","output":"89058885\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt};
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;
use algo_lib::{dynamic_value, transparent_wrapper};
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::{Add, AddAssign, Mul};

type PreCalc = ();

transparent_wrapper!(Matrix<T> = Arr2d<T>, derive Clone, Eq, PartialEq, Default);

impl<T: Zero + One + Clone> Matrix<T> {
    pub fn zero(n: usize, m: usize) -> Self {
        Self(Arr2d::new(n, m, T::zero()))
    }

    pub fn ident(n: usize) -> Self {
        Self(Arr2d::with_gen(n, n, |i, j| {
            if i == j {
                T::one()
            } else {
                T::zero()
            }
        }))
    }
}

impl<T: Copy> Matrix<T> {
    pub fn column(arr: &[T]) -> Self {
        Self(Arr2d::with_gen(arr.len(), 1, |i, _| arr[i]))
    }

    pub fn row(arr: &[T]) -> Self {
        Self(Arr2d::with_gen(1, arr.len(), |_, i| arr[i]))
    }

    pub fn new(arr: &[&[T]]) -> Self {
        for a in arr {
            assert_eq!(a.len(), arr[0].len());
        }
        Self(Arr2d::with_gen(arr.len(), arr[0].len(), |i, j| arr[i][j]))
    }
}

impl<T: Add<Output = T> + AddAssign + Mul<Output = T> + One + Zero + Copy + BaseModInt<u32>>
    Matrix<T>
{
    pub fn mult(&self, a: &Matrix<T>) -> Self {
        let mut res = Self::zero(self.d1(), a.d2());
        Self::do_mult(&mut res, self, a, &mut [], &mut []);
        res
    }

    pub fn do_mult(&mut self, a: &Matrix<T>, b: &Matrix<T>, x: &mut [i64], y: &mut [i64]) {
        assert_eq!(self.d1(), a.d1());
        assert_eq!(a.d2(), b.d1());
        assert_eq!(b.d2(), self.d2());
        self.fill(T::zero());
        for i in 0..self.d1() {
            for k in 0..b.d2() {
                for j in 0..a.d2() {
                    x[j] = a[(i, j)].value() as i64;
                    y[j] = b[(j, k)].value() as i64;
                }
                let m = T::module() as i64;
                const INF: i64 = i64::MAX - 1_000_000_000_000_000_000;
                #[target_feature(enable = "avx2")]
                unsafe fn dot_product(x: &[i64], y: &[i64], m: i64) -> i64 {
                    let mut res = 0;
                    for i in 0..x.len() {
                        res += x[i] * y[i];
                        if res >= INF {
                            res %= m;
                        }
                    }
                    res
                }
                let sum = unsafe { dot_product(x, y, m) };
                // for j in 0..a.d2() {
                //     sum += x[j] * y[j];
                // }
                self[(i, k)] = T::from((sum % T::module() as i64) as u32);
                // for j in 0..a.d2() {
                //     self[(i, k)] += a[(i, j)] * b[(j, k)];
                // }
            }
        }
    }

    pub fn add(&mut self, a: &Matrix<T>, b: &Matrix<T>) {
        assert_eq!(self.d1(), a.d1());
        assert_eq!(self.d2(), a.d2());
        assert_eq!(self.d1(), b.d1());
        assert_eq!(self.d2(), b.d2());
        for i in 0..self.d1() {
            for j in 0..self.d2() {
                self[(i, j)] = a[(i, j)] + b[(i, j)];
            }
        }
    }

    pub fn add_to(&mut self, a: &Matrix<T>) {
        assert_eq!(self.d1(), a.d1());
        assert_eq!(self.d2(), a.d2());
        for i in 0..self.d1() {
            for j in 0..self.d2() {
                self[(i, j)] += a[(i, j)];
            }
        }
    }

    pub fn power(&self, n: usize) -> Matrix<T> {
        assert_eq!(self.d1(), self.d2());
        let mut a = vec![0; self.d1()];
        let mut b = vec![0; self.d1()];
        let mut res = Self::ident(self.d1());
        let mut temp = Self::ident(self.d1());
        Self::do_power(self, &mut res, &mut temp, n, &mut a, &mut b);
        res
    }

    fn do_power(
        a: &Matrix<T>,
        res: &mut Matrix<T>,
        temp: &mut Matrix<T>,
        n: usize,
        x: &mut [i64],
        y: &mut [i64],
    ) {
        if n != 0 {
            if (n & 1) == 0 {
                Self::do_power(a, temp, res, n >> 1, x, y);
                res.do_mult(temp, temp, x, y);
            } else {
                Self::do_power(a, temp, res, n - 1, x, y);
                res.do_mult(temp, a, x, y);
            }
        }
    }

    pub fn sum_power(&self, n: usize) -> Self {
        assert_eq!(self.d1(), self.d2());
        let mut res = Self::zero(self.d1(), self.d2());
        let mut temp = Self::zero(self.d1(), self.d2());
        let mut pw = Self::ident(self.d1());
        let mut temp_pw = Self::ident(self.d1());
        Self::do_sum_power(self, &mut res, &mut temp, &mut pw, &mut temp_pw, n);
        res
    }

    fn do_sum_power(
        a: &Matrix<T>,
        res: &mut Matrix<T>,
        temp: &mut Matrix<T>,
        pw: &mut Matrix<T>,
        temp_pw: &mut Matrix<T>,
        n: usize,
    ) {
        if n != 0 {
            if (n & 1) == 0 {
                Self::do_sum_power(a, temp, res, temp_pw, pw, n >> 1);
                pw.do_mult(temp_pw, temp_pw, &mut [], &mut []);
                for i in 0..pw.d1() {
                    temp_pw[(i, i)] += T::one();
                }
                res.do_mult(temp, temp_pw, &mut [], &mut []);
            } else {
                Self::do_sum_power(a, res, temp, temp_pw, pw, n - 1);
                pw.do_mult(temp_pw, a, &mut [], &mut []);
                res.add_to(temp_pw);
            }
        }
    }
}

impl<T> From<Arr2d<T>> for Matrix<T> {
    fn from(a: Arr2d<T>) -> Self {
        Self(a)
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn multiply_2x2() {
        let a = Matrix::new(&[&[1i64, 2], &[3, 4]]);
        let b = Matrix::new(&[&[5i64, 6], &[7, 8]]);
        let c = a.mult(&b);
        assert!(c == Matrix::new(&[&[19i64, 22], &[43, 50]]));
    }

    #[test]
    fn power_identity() {
        let id = Matrix::<i64>::ident(3);
        assert!(id.power(100) == Matrix::ident(3));
    }

    #[test]
    fn power_fib() {
        let m = Matrix::new(&[&[1i64, 1], &[1, 0]]);
        let r = m.power(10);
        // [[1,1],[1,0]]^10 = [[fib(11), fib(10)], [fib(10), fib(9)]]
        assert!(r == Matrix::new(&[&[89i64, 55], &[55, 34]]));
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let modulo = input.read_unsigned();

    dynamic_value!(Modulo: u32 = modulo);
    type Mod = ModInt<Modulo>;
    let mut mat = Matrix::<Mod>::zero(2 * m + 1, 2 * m + 1);
    for i in 0..m {
        for j in i..m {
            for mut a in i..=j {
                if a == i {
                    a += m;
                }
                mat[(a, 2 * m)] += 1;
                for b in i..=j {
                    mat[(a, b + m)] += 1;
                    if b == i {
                        mat[(a, b)] += 1;
                    }
                }
            }
        }
    }
    mat[(2 * m, 2 * m)] += 1;
    let ans = mat.power(n);
    out.print_line(ans[(m, 2 * m)]);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
