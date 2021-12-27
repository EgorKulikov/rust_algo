//{"name":"D. Перемешивание","group":"Codeforces - Educational Codeforces Round 120 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1622/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7 2\n1100110\n","output":"16\n"},{"input":"5 0\n10010\n","output":"1\n"},{"input":"8 1\n10001000\n","output":"10\n"},{"input":"10 8\n0010011000\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPeremeshivanie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let s: Str = input.read();

    type Mod = ModIntF;
    let c = Combinations::new(n + 1);
    if s.iter().filter(|c| *c == b'1').count() < k || k == 0 {
        out_line!(1);
        return;
    }
    let mut ans = Mod::zero();
    let mut end = s
        .iter()
        .enumerate()
        .filter(|(_, c)| *c == b'1')
        .skip(k)
        .map(|(i, _)| i)
        .next()
        .unwrap_or(n);
    for (i, d) in s.iter().enumerate() {
        if d == b'0' {
            ans += c.c(end - i - 1, k - 1);
        } else {
            if end == n {
                ans += c.c(end - i, k);
                break;
            }
            if end - i - 1 >= k {
                ans += c.c(end - i - 1, k);
            }
            loop {
                end += 1;
                if end == n || s[end] == b'1' {
                    break;
                }
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
