//{"name":"Weird Fibbo","group":"CodeChef - CDVN2023","url":"https://www.codechef.com/CDVN2023/problems/WEIRD_FIBBO","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 3 0\n2 3 5\n2 2 3\n2 2 4\n","output":"2\n63743\n35\n775\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WeirdFibbo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::{ModInt, ModInt7};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::{out_line, value};

value!(Val6: i32 = 1_000_000_006);
type Mod6 = ModInt<i32, Val6>;

fn solve(input: &mut Input, _test_case: usize, mat: &Matrix<Mod6>) {
    let a = input.read_unsigned();
    let b = input.read_unsigned();
    let n = input.read_size();

    if n == 0 {
        out_line!(a);
        return;
    }
    if n == 1 {
        out_line!(b);
        return;
    }
    let a_ones = a.count_ones().into_i32();
    let a_zeros = (a.count_zeros() - a.leading_zeros()).into_i32();
    let b_ones = b.count_ones().into_i32();
    let b_zeros = (b.count_zeros() - b.leading_zeros()).into_i32();

    let calc_impl = |n: usize, f0: Mod6, f1: Mod6| -> Mod6 {
        if n == 0 {
            return f0;
        }
        let res = mat.power(n);
        f1 * res[(1, 0)] + f0 * res[(1, 1)]
    };

    let calc = |n: usize| -> (Mod6, Mod6) {
        (
            calc_impl(n, Mod6::new(a_zeros), Mod6::new(b_zeros)),
            calc_impl(n, Mod6::new(a_ones), Mod6::new(b_ones)),
        )
    };

    let (n2_zeros, n2_ones) = calc(n - 2);
    let (n1_zeros, n1_ones) = calc(n - 1);

    type Mod = ModInt7;
    let ans = Mod::new(2).power((n1_ones + n1_zeros + n2_ones + n2_zeros).val())
        - Mod::new(2).power((n1_ones + n1_zeros + n2_zeros).val())
        + Mod::new(2).power(n1_ones.val())
        - Mod::one();
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let mat = Matrix::new(&[&[Mod6::one(), Mod6::one()], &[Mod6::one(), Mod6::zero()]]);
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1, &mat);
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
