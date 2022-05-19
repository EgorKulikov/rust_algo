//{"name":"H. Сложный разрез","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n00000\n01101\n0111011001011\n000111100111110\n","output":"-1\n\n3\n1 3\n4 4\n5 5\n\n8\n1 2\n3 3\n4 4\n5 6\n7 7\n8 10\n11 12\n13 13\n\n5\n1 5\n6 7\n8 11\n12 14\n15 15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HSlozhniiRazrez"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::ops::Shl;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let ans = solve_impl(s);
    match ans {
        None => {
            out_line!(-1);
        }
        Some(ans) => {
            out_line!(ans.len());
            output().print_per_line(&ans);
        }
    }
}

#[test]
fn stress() {
    for i in 1..=20 {
        for j in 1..1.shl(i) {
            let mut s = Str::new();
            for k in 0..i {
                if j.is_set(k) {
                    s += b'1';
                } else {
                    s += b'0';
                }
            }
            solve_impl(s);
        }
    }
}

fn solve_impl(s: Str) -> Option<Vec<(usize, usize)>> {
    let ones = s.iter().filter(|&c| c == b'1').count();
    if ones == 0 {
        return None;
    }
    let mut target = if ones.is_power_of_two() {
        ones
    } else {
        ones.next_power_of_two()
    };
    loop {
        let was_target = target;
        let mut rem = ones;
        let mut ans = Vec::new();
        let mut start = 1;
        let mut cur = 0;
        for (i, c) in s.iter().enumerate() {
            let mut cand = cur * 2;
            if c == b'1' {
                rem -= 1;
                cand += 1;
            }
            if cand + rem > target {
                ans.push((start, i));
                target -= cur;
                start = i + 1;
                cur = if c == b'1' { 1 } else { 0 };
            } else {
                cur = cand;
            }
        }
        if target == cur {
            ans.push((start, s.len()));
            break Some(ans);
        }
        target = 2 * was_target;
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
