//{"name":"F. Странные инструкции","group":"Codeforces - Hello 2022","url":"https://codeforces.com/contest/1621/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 2 2 1\n01101\n6 4 3 5\n110001\n6 3 2 1\n011110\n","output":"3\n11\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FStrannieInstruktsii"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let s: Str = input.read();

    let mut near_ones = 0;
    let mut near_zeroes = 0;
    let mut group = 0;
    let mut groups = Vec::new();
    for i in 1..n {
        if s[i] == s[i - 1] {
            if s[i] == b'0' {
                near_zeroes += 1;
            } else {
                near_ones += 1;
            }
            group += 1;
        } else {
            if s[i - 1] == b'0' && group != i - 1 {
                groups.push(group);
            }
            group = 0;
        }
    }
    let mut side_zeroes = 0;
    if s[0] == b'0' {
        side_zeroes += 1;
    }
    if s[n - 1] == b'0' {
        side_zeroes += 1;
    }
    if near_ones >= near_zeroes {
        let mut ans = near_zeroes * (a + b);
        if near_ones > near_zeroes {
            ans += b;
        }
        if c < b {
            ans += (b - c) * groups.len().into_i64();
            if near_ones > near_zeroes + 1 && side_zeroes > 0 {
                ans += b - c;
            }
            if near_ones > near_zeroes + 2 && side_zeroes > 1 {
                ans += b - c;
            }
        }
        out_line!(ans);
    } else {
        let mut ans = near_ones * (a + b) + a;
        if c < b {
            groups.sort();
            let mut sum = 0;
            for g in groups {
                sum += g;
                if sum.into_i64() <= near_ones {
                    ans += b - c;
                } else {
                    break;
                }
            }
        }
        out_line!(ans);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
