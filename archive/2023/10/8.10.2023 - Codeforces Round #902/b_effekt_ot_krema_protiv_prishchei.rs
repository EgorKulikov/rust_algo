//{"name":"B. Эффект от крема против прыщей","group":"Codeforces - Codeforces Round 902 (Div. 1, based on COMPFEST 15 - Final Round)","url":"https://codeforces.com/contest/1876/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n19 14 19 9\n","output":"265\n"},{"input":"1\n0\n","output":"0\n"},{"input":"15\n90000 9000 99000 900 90900 9900 99900 90 90090 9090 99090 990 90990 9990 99990\n","output":"266012571\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BEffektOtKremaProtivPrishchei"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::powers;
use algo_lib::numbers::primes::factorize::all_divisors;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    type Mod = ModIntF;
    let mut free = n;
    let mut done = BitSet::new(n);
    let divisors = all_divisors::<usize>(n + 1, false);
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| Reverse(a[i]));
    let mut ans = Mod::zero();
    let twos = powers(Mod::new(2), n + 1);

    for i in order {
        let mut enablers = 0;
        for &j in &divisors[i + 1] {
            if !done[j - 1] {
                enablers += 1;
            }
        }
        ans += Mod::new(a[i]) * (twos[free] - twos[free - enablers]);
        for &j in &divisors[i + 1] {
            if !done[j - 1] {
                done.set(j - 1);
                free -= 1;
            }
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

#[test]
fn test() {
    rust_competitive_helper_util::build::build();
}

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
