//{"name":"G - evall","group":"AtCoder - AtCoder Beginner Contest 338","url":"https://atcoder.jp/contests/abc338/tasks/abc338_g","interactive":false,"timeLimit":2000,"tests":[{"input":"1+2*34\n","output":"197\n"},{"input":"338*3338*33338*333338+3333338*33333338+333333338\n","output":"527930018\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GEvall"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    type Mod = ModIntF;
    let mut sum_add = Mod::zero();
    let mut add_qty = 0;
    let mut sum_mult = Mod::zero();
    let mut mult_qty = 0;
    let mut sum_cur = Mod::zero();
    let mut cur_qty = 0;
    let mut cur = Mod::zero();
    let mut mult = Mod::one();
    let mut ans = Mod::zero();

    for c in s {
        match c {
            b'0'..=b'9' => {
                let d = Mod::new((c - b'0') as i32);
                cur = cur * Mod::new(10) + d;
                ans += sum_add + Mod::from_index(add_qty) * cur * mult;
                ans += sum_mult * cur;
                sum_cur *= Mod::new(10);
                cur_qty += 1;
                sum_cur += d * Mod::from_index(cur_qty);
                ans += sum_cur;
            }
            b'*' => {
                mult *= cur;
                sum_mult *= cur;
                sum_mult += sum_cur;
                mult_qty += cur_qty;
                cur = Mod::zero();
                sum_cur = Mod::zero();
                cur_qty = 0;
            }
            b'+' => {
                mult *= cur;
                sum_mult *= cur;
                sum_mult += sum_cur;
                mult_qty += cur_qty;
                cur = Mod::zero();
                sum_cur = Mod::zero();
                cur_qty = 0;

                sum_add += mult * Mod::from_index(add_qty);
                sum_add += sum_mult;
                add_qty += mult_qty;
                mult = Mod::one();
                sum_mult = Mod::zero();
                mult_qty = 0;
            }
            _ => unreachable!(),
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
    //    tester::stress_test();
}
//END MAIN
