//{"name":"E. Left is Always Right","group":"Codeforces - Pinely Round 5 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2161/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 3\n0??0?\n7 7\n1??1??1\n9 5\n?????????\n","output":"3\n15\n46\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    type Mod = ModIntF;
    fn go(n: usize, k: usize, s: Str) -> Mod {
        let mut any = k - 1;
        let mut zero = 0;
        let mut one = 0;
        let mut tp = vec![2; k - 1];
        let mut can_be_zero = vec![true];
        let mut can_be_one = vec![true];
        for i in 0..n {
            can_be_zero.push(can_be_zero[i] && s[i] != b'1');
            can_be_one.push(can_be_one[i] && s[i] != b'0');
        }
        let mut ans = Mod::zero();
        let c = Combinations::<Mod>::new(k);
        for i in (0..n).rev() {
            let ctp = tp[i % (k - 1)];
            match s[i] {
                b'0' => {
                    if ctp == 2 {
                        tp[i % (k - 1)] = 0;
                        any -= 1;
                        zero += 1;
                    } else if ctp == 1 {
                        break;
                    }
                }
                b'1' => {
                    if ctp == 2 {
                        tp[i % (k - 1)] = 1;
                        any -= 1;
                        one += 1;
                    } else if ctp == 0 {
                        break;
                    }
                }
                b'?' => {}
                _ => unreachable!(),
            }
            let ctp = tp[i % (k - 1)];
            if i > n - k || i == 0 {
                continue;
            }
            if can_be_zero[i] && ctp != 0 {
                let cur_zero = zero;
                let mut cur_any = any;
                let mut cur_one = one;
                if ctp == 2 {
                    cur_any -= 1;
                    cur_one += 1;
                }
                if cur_one <= (k - 1) / 2 && cur_zero <= (k - 1) / 2 {
                    ans += c.c(cur_any, (k - 1) / 2 - cur_one);
                }
            }
            if can_be_one[i] && ctp != 1 {
                let cur_one = one;
                let mut cur_any = any;
                let mut cur_zero = zero;
                if ctp == 2 {
                    cur_any -= 1;
                    cur_zero += 1;
                }
                if cur_one <= (k - 1) / 2 && cur_zero <= (k - 1) / 2 {
                    ans += c.c(cur_any, (k - 1) / 2 - cur_zero);
                }
            }
        }
        if can_be_zero[n - k + 1] {
            let mut ones = 0;
            let mut any = 0;
            for i in 0..k - 1 {
                if s[Back(i)] == b'1' {
                    ones += 1;
                } else if s[Back(i)] == b'?' {
                    any += 1;
                }
            }
            // let limit = if s[Back(0)] == b'1' {
            //     (k - 1) / 2
            // } else {
            //     (k - 1) / 2 - 1
            // };
            let limit = (k - 1) / 2;
            if ones <= limit {
                for add_ones in 0..=any.min(limit - ones) {
                    ans += c.c(any, add_ones);
                }
            }
        }
        if can_be_one[n - k + 1] {
            let mut zeros = 0;
            let mut any = 0;
            for i in 0..k - 1 {
                if s[Back(i)] == b'0' {
                    zeros += 1;
                } else if s[Back(i)] == b'?' {
                    any += 1;
                }
            }
            // let limit = if s[Back(0)] == b'0' {
            //     (k - 1) / 2
            // } else {
            //     (k - 1) / 2 - 1
            // };
            let limit = (k - 1) / 2;
            if zeros <= limit {
                for add_zeros in 0..=any.min(limit - zeros) {
                    ans += c.c(any, add_zeros);
                }
            }
        }
        ans
    }

    let mut ans = Mod::zero();
    // if s[Back(0)] == b'?' {
    //     let mut s0 = s.clone();
    //     s0[Back(0)] = b'0';
    //     ans += go(n, k, s0);
    //     let mut s1 = s;
    //     s1[Back(0)] = b'1';
    //     ans += go(n, k, s1);
    // } else {
    ans += go(n, k, s);
    // }

    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
