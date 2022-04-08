//{"name":"G. Mismatch","group":"Yandex - Stage 12: Grand Prix of Grushevka","url":"https://official.contest.yandex.com/opencupXXII/contest/35268/problems/G/","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n0 1 2\n","output":"1 3 1\n"},{"input":"6\n1 2 2 7 6 7\n","output":"0 3 9 10 5 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMismatch"}}}

use algo_lib::collections::vec_ext::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::prime_fft::PrimeFFT;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut qty = a.qty_bound(1 << 19);
    for i in 0..19 {
        for j in 0..qty.len() {
            if j.is_set(i) {
                let q = qty[j];
                qty[j.without_bit(i)] += q;
            }
        }
    }
    type Mod = ModIntF;
    let c: Combinations<Mod> = Combinations::new(n + 1);
    let mut a = vec![Mod::zero(); n + 1];
    for (i, q) in qty.into_iter().enumerate() {
        if i.count_ones() % 2 == 0 {
            a[n - q] += c.fact(q);
        } else {
            a[n - q] -= c.fact(q);
        }
    }
    let mut b = Vec::with_capacity(n + 1);
    for i in 0..=n {
        b.push(c.inv_fact(i));
    }
    let mut fft = PrimeFFT::new();
    let res = fft.multiply(&a, &b);
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        ans.push(res[n - i - 1] * c.inv_fact(i + 1));
    }
    out_line!(ans);
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
