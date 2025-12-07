//{"name":"C1. Красивые узоры (простая версия)","group":"Codeforces - Codeforces Round 1069 (Div. 1)","url":"https://codeforces.com/contest/2174/problem/C1","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2 101\n5 1 999999937\n100 23190 3214373\n","output":"57\n225\n2347147\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::num_utils::{powers, UpperDiv};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_unsigned();

    dynamic_value!(Modulo: u32 = p);
    type Mod = ModInt<Modulo>;
    let pw = powers(Mod::from(m).inv().unwrap(), n + 1);
    let mut ans = Mod::zero();
    let mut single = Mod::zero();
    let mut all = Mod::zero();
    let mut s = vec![Mod::zero()];
    let mut a = vec![Mod::zero()];
    for i in 1..=n {
        single += pw[i / 2];
        all += single;
        s.push(single);
        a.push(all);
    }
    for i in 1..n {
        ans += a[i] * s[n - i] * 2;
    }
    for len in 1..=n {
        ans += pw[len / 2] * (len.upper_div(2) * 2 - 1) * (n - len + 1);
    }
    for len in 1..=n {
        for len2 in 1..len {
            let mut q = len - len2 + 1;
            if len % 2 == len2 % 2 {
                q -= 1;
            }
            ans += pw[len / 2 + len2 / 2] * q * (n - len + 1) * 2;
        }
    }
    for total_len in 3..=n {
        for o1 in 1..=2 {
            for o2 in 1..=2 {
                let mut start = o1 + o2 - total_len % 2;
                if start > 2 {
                    start -= 2;
                }
                for common in (start..=total_len - 2).step_by(2) {
                    let min_x = (common + 2 - o1) / 2;
                    let max_x = (total_len - o1 - 1) / 2;
                    assert!(min_x <= max_x + 1);
                    let len1 = 2 * min_x + o1;
                    let len2 = total_len + common - len1;
                    ans += pw[len1 / 2 + len2 / 2] * (n - total_len + 1) * (max_x + 1 - min_x) * 2;
                }
            }
        }
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
