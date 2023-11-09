//{"name":"B. Beauty index","group":"Yandex - Yandex Cup 2023 — Algorithm — Semifinal","url":"https://contest.yandex.com/contest/54740/problems/B/","interactive":false,"timeLimit":1000,"tests":[{"input":"13 2\n","output":"500\n"},{"input":"2023 3\n","output":"688826511\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBeautyIndex"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut a = input.read_long();
    let p = input.read_long();

    let mut d = Vec::new();
    while a > 0 {
        d.push(a % p);
        a /= p;
    }
    type Mod = ModInt7;
    let mut mult = Mod::one();
    let mut ans = Mod::zero();
    let mut qty = Mod::one();
    let mut num_zero = 0;
    for &i in &d {
        let d = Mod::new_from_wide(i + 1);
        if d == Mod::zero() {
            num_zero += 1;
        } else {
            qty *= d;
        }
    }
    for &i in &d {
        let n = Mod::new_from_wide(i);
        let d = Mod::new_from_wide(i + 1);
        let sum_sq = n * (n + Mod::one()) * (n + n + Mod::one()) / Mod::new(6);
        if d == Mod::zero() {
            if num_zero == 1 {
                ans += sum_sq * qty * mult * mult;
            }
        } else if num_zero == 0 {
            ans += sum_sq * qty / d * mult * mult;
        }
        mult *= Mod::new_from_wide(p);
    }
    mult = Mod::one();
    for (idx, &i) in d.iter().enumerate() {
        let dd = Mod::new_from_wide(i + 1);
        if dd == Mod::zero() {
            num_zero -= 1;
        } else {
            qty /= dd;
        }
        let mut mult2 = Mod::one();
        let n1 = Mod::new_from_wide(i);
        let sum1 = n1 * (n1 + Mod::one()) / Mod::new(2);
        for &j in d.iter().take(idx) {
            let dd = Mod::new_from_wide(j + 1);
            if dd == Mod::zero() {
                num_zero -= 1;
            } else {
                qty /= dd;
            }
            let n2 = Mod::new_from_wide(j);
            let sum2 = n2 * (n2 + Mod::one()) / Mod::new(2);
            if num_zero == 0 {
                ans += sum1 * sum2 * qty * mult * mult2 * Mod::new(2);
            }
            if dd == Mod::zero() {
                num_zero += 1;
            } else {
                qty *= dd;
            }
            mult2 *= Mod::new_from_wide(p);
        }
        if dd == Mod::zero() {
            num_zero += 1;
        } else {
            qty *= dd;
        }
        mult *= Mod::new_from_wide(p);
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
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
