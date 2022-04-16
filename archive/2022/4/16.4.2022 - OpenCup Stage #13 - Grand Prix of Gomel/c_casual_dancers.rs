//{"name":"C. Casual Dancers","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/C/","interactive":false,"timeLimit":4000,"tests":[{"input":"0 0 0\n1\n58\n","output":"1\n"},{"input":"1 2 2\n1\n100\n","output":"332748119\n"},{"input":"5 2 3\n4\n50\n","output":"160212060\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCasualDancers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::invertable::Invertable;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::prime_fft::PrimeFFT;
use algo_lib::out_line;

type Mod = ModIntF;

fn solve_impl(mut x: Vec<i32>, k: usize) -> Mod {
    let mut fft: PrimeFFT<Mod> = PrimeFFT::new();
    let third = Mod::new(3).inv().unwrap();
    let mut rec = RecursiveFunction::new(|f, n: usize| -> Vec<Mod> {
        if n == 0 {
            vec![Mod::one()]
        } else if n % 2 == 0 {
            let res = f.call(n / 2);
            fft.multiply(&res, &res)
        } else {
            let res = f.call(n - 1);
            fft.multiply(&res, &[third, third, third])
        }
    });
    let res = rec.call(k);
    let mut sum = Vec::with_capacity(2 * k + 1);
    let mut sum_mult = Vec::with_capacity(2 * k + 1);
    let mut cur_sum = Mod::zero();
    let mut cur_sum_mult = Mod::zero();
    for (i, &v) in res.iter().enumerate() {
        cur_sum += v;
        cur_sum_mult += v * Mod::from_index(i);
        sum.push(cur_sum);
        sum_mult.push(cur_sum_mult);
    }
    x.sort_unstable();
    let mut ans = Mod::zero();
    for i in 0..3 {
        for j in 0..i {
            let delta = (x[i] - x[j]).into_usize();
            if delta >= k {
                ans += Mod::from_index(delta);
                continue;
            }
            let mut res = sum_mult[2 * k]
                - sum_mult[k - delta]
                - Mod::from_index(k - delta) * (sum[2 * k] - sum[k - delta]);
            res += sum_mult[2 * k]
                - sum_mult[k + delta]
                - Mod::from_index(k + delta) * (sum[2 * k] - sum[k + delta]);
            ans += res;
        }
    }
    ans / Mod::new(2)
}

fn solve(input: &mut Input, _test_case: usize) {
    let x = input.read_int_vec(3);
    let k = input.read_usize();
    let _p = input.read_int();

    out_line!(solve_impl(x, k));
}

#[test]
fn test() {
    solve_impl(vec![-100000, 0, 100000], 200000);
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
