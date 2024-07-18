//{"name":"G. Minecraft","group":"Codeforces - Codeforces Round 959 при поддержке NEAR (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1994/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 5\n01011\n01110\n00110\n01100\n01111\n2 8\n00101001\n10111111\n10011110\n5 4\n0101\n0010\n0000\n0000\n0010\n0011\n6 5\n00011\n10110\n11001\n01010\n11100\n10011\n10000\n","output":"01110\n10011010\n0010\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMinecraft"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();
    let a = input.read_str_vec(n);

    let mut ones = Vec::with_capacity(k);
    for i in 0..k {
        let mut cur = 0;
        for s in &a {
            if s[i] == b'1' {
                cur += 1;
            }
        }
        ones.push(cur);
    }
    let mut next = Arr2d::new(k + 1, n, None);
    next[(k, 0)] = Some((0, 0));
    for i in (0..k).rev() {
        for j in 0..n {
            if next[(i + 1, j)].is_some() {
                let total0 = j + ones[i];
                let total1 = j + (n - ones[i]);
                for (bit, total) in [total0, total1].into_iter().enumerate() {
                    if total % 2 != (s[i] - b'0') as usize {
                        continue;
                    }
                    if next[(i, total / 2)].is_none() {
                        next[(i, total / 2)] = Some((bit, j));
                    }
                }
            }
        }
    }
    if next[(0, 0)].is_none() {
        out.print_line(-1);
        return;
    }
    let mut res = Str::with_capacity(k);
    let mut carry = 0;
    for i in 0..k {
        let (bit, new_carry) = next[(i, carry)].unwrap();
        res.push(b'0' + (bit as u8));
        carry = new_carry;
    }
    out.print_line(res);
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
}
//END MAIN
