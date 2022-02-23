//{"name":"M. Math String","group":"Yandex - Stage 10: Grand Prix of Kyoto","url":"https://official.contest.yandex.ru/opencupXXII/contest/35263/problems/M/","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n","output":"45\n"},{"input":"3\n","output":"407430\n"},{"input":"1000000000000000000\n","output":"493565653\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MMathString"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use std::ops::{Deref, DerefMut};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();

    type Mod = ModIntF;
    struct Matrix(Arr2d<Mod>);
    impl Deref for Matrix {
        type Target = Arr2d<Mod>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for Matrix {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl Matrix {
        fn new() -> Self {
            Self(Arr2d::new(8, 8, Mod::zero()))
        }

        fn mult(c: &mut Matrix, a: &Matrix, b: &Matrix) {
            for i in 0..c.d1() {
                for j in 0..c.d2() {
                    c[(i, j)] = Mod::zero();
                    for k in 0..b.d1() {
                        c[(i, j)] += a[(i, k)] * b[(k, j)];
                    }
                }
            }
        }

        fn power(&mut self, temp: &mut Matrix, n: i64, m: &Matrix) {
            if n == 0 {
                self.fill(Mod::zero());
                for i in 0..self.d1() {
                    self[(i, i)] = Mod::one();
                }
                return;
            }
            if n % 2 == 0 {
                temp.power(self, n >> 1, m);
                Self::mult(self, temp, temp);
            } else {
                temp.power(self, n - 1, m);
                Self::mult(self, temp, m);
            }
        }
    }

    let mut mat = Matrix::new();
    mat[(0, 3)] += Mod::one();
    mat[(0, 5)] += Mod::one();
    mat[(1, 7)] += Mod::one();
    mat[(6, 7)] += Mod::one();
    mat[(0, 3)] += Mod::one();
    mat[(1, 5)] += Mod::one();
    mat[(6, 7)] += Mod::one();
    mat[(3, 0)] += Mod::new(9);
    mat[(3, 3)] += Mod::new(9);
    mat[(4, 1)] += Mod::new(9);
    mat[(4, 4)] += Mod::new(9);
    mat[(5, 1)] += Mod::new(45);
    mat[(5, 2)] += Mod::new(90);
    mat[(5, 4)] += Mod::new(45);
    mat[(5, 5)] += Mod::new(90);
    mat[(7, 6)] += Mod::new(9);
    mat[(7, 7)] += Mod::new(9);
    let mut ans = Matrix::new();
    let mut temp = Matrix::new();
    ans.power(&mut temp, n, &mat);
    let mut res = Mod::zero();
    for i in [1, 6] {
        for j in [3, 5] {
            res += ans[(j, i)];
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
