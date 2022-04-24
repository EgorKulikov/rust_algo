//{"name":"Palindromic Factors","group":"Google Coding Competitions - Round B 2022 - Kick Start 2022","url":"https://codingcompetitions.withgoogle.com/kickstart/round/00000000008caa74/0000000000acee89","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\n10\n144\n242\n","output":"Case #1: 4\nCase #2: 3\nCase #3: 7\nCase #4: 6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PalindromicFactors"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let a = input.read_long();

    let mut ans = 0;
    let mut i = 1;
    fn palindrome(x: i64) -> i32 {
        let s = x.to_string();
        let mut t = s.clone().into_bytes();
        t.reverse();
        if s.into_bytes() == t {
            1
        } else {
            0
        }
    }
    while i * i <= a {
        if a % i == 0 {
            ans += palindrome(i);
            if i * i != a {
                ans += palindrome(a / i);
            }
        }
        i += 1;
    }
    out_line!(format!("Case #{}:", test_case), ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
