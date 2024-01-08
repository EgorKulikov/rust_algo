//{"name":"E. Подсчёт префиксов","group":"Codeforces - Hello 2024","url":"https://codeforces.com/contest/1919/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n0\n1\n1\n3\n-1 1 2\n5\n-1 0 0 1 1\n5\n-4 -3 -3 -2 -1\n","output":"0\n1\n0\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPodschyotPrefiksov"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization4;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let p = input.read_int_vec(n);

    let mut qty = vec![0; 2 * n + 1];
    for i in p {
        qty[(i + (n as i32)) as usize] += 1;
    }
    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + 1);
    let zeroes = qty[n];
    let positive = qty[n + 1..].to_vec();
    let mut negative = qty[..n].to_vec();
    negative.reverse();
    let mut mem = Memoization4::new(
        |mem, tail: bool, is_positive: bool, q: usize, position: usize| -> Mod {
            if position == n {
                if q == 0 {
                    return Mod::one();
                } else {
                    return Mod::zero();
                }
            }
            let val = if is_positive {
                positive[position]
            } else {
                negative[position]
            };
            if q == 0 {
                if val == 0 {
                    return mem.call(false, is_positive, 0, position + 1);
                }
                return Mod::zero();
            }
            if val < q {
                return Mod::zero();
            }
            if !tail {
                return c.c(val - 1, q - 1) * mem.call(false, is_positive, val - q, position + 1);
            }
            let mut ans = c.c(val - 1, q - 1) * mem.call(false, is_positive, val - q, position + 1);
            ans += c.c(val - 1, q - 1) * mem.call(true, is_positive, val - q + 1, position + 1);
            ans
        },
    );
    let mut ans = Mod::zero();
    for (ends_postive, ends_negative) in [(true, false), (false, true), (false, false)] {
        let pos = if ends_postive || ends_negative {
            zeroes + 1
        } else {
            zeroes
        };
        for i in 0..=pos.min(positive[0]) {
            if ends_postive && i == 0 {
                continue;
            }
            let neg = pos - i;
            if ends_negative && neg == 0 || neg > negative[0] {
                continue;
            }
            let base = if ends_postive {
                c.c(pos - 1, i - 1)
            } else if ends_negative {
                c.c(pos - 1, i)
            } else {
                c.c(pos, i)
            };
            ans +=
                base * mem.call(ends_postive, true, i, 0) * mem.call(ends_negative, false, neg, 0);
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
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
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

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
