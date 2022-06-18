//{"name":"F. Я могу ошибаться","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n1\n2\n10\n3\n101\n4\n1000\n5\n11010\n6\n110000\n20\n01000010001010011000\n","output":"0\n1\n1\n3\n2\n2\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FYaMoguOshibatsya"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut s: Str = input.read();

    if s.iter().filter(|&id| id == b'1').count() > s.iter().filter(|&id| id == b'0').count() {
        s.reverse();
        for c in s.iter_mut() {
            if *c == b'0' {
                *c = b'1';
            } else {
                *c = b'0';
            }
        }
    }
    let mut cur_delta = 0i32;
    let mut ones = 0i32;
    let mut heap = BinaryHeap::with_capacity(n);
    for (i, c) in s.iter().enumerate() {
        if c == b'1' {
            cur_delta += 1;
            ones += 1;
        } else {
            cur_delta -= 1;
        }
        heap.push((cur_delta, i, ones));
    }
    let first = s.iter().find(b'1');
    let mut first = match first {
        None => {
            out_line!(0);
            return;
        }
        Some(first) => first.into_i32(),
    };
    if cur_delta == n.into_i32() - 2 * first {
        out_line!(0);
        return;
    }
    let mut ans = 0;
    let mut can_be_sorted_to = first;
    let mut num_ones_can_be = 1;
    while let Some((delta, pos, ones)) = heap.pop() {
        if pos.into_i32() <= can_be_sorted_to {
            continue;
        }
        if delta + first >= 0 {
            can_be_sorted_to = pos.into_i32();
            num_ones_can_be = ones;
        } else {
            ans += 1;
            first = can_be_sorted_to - num_ones_can_be + 1;
            assert!(delta + first >= 0);
            can_be_sorted_to = pos.into_i32();
            num_ones_can_be = ones;
        }
    }
    out_line!(ans + 1);
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
