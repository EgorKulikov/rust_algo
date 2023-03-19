//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let mut q: VecDeque<_> = input.read_size_vec(98).into();

    let mut hand = VecDeque::with_capacity(8);
    let mut up1 = vec![1];
    let mut up2 = vec![1];
    let mut down1 = vec![100];
    let mut down2 = vec![100];
    while hand.len() < 8 {
        hand.push_back(q.pop_front().unwrap());
    }

    while !hand.is_empty() {
        let mut cur = None;
        for (i, &c) in hand.iter().enumerate() {
            if c + 10 == *up1.last().unwrap() {
                cur = Some((i, 0));
                break;
            }
            if c + 10 == *up2.last().unwrap() {
                cur = Some((i, 1));
                break;
            }
            if c == *down1.last().unwrap() + 10 {
                cur = Some((i, 2));
                break;
            }
            if c == *down2.last().unwrap() + 10 {
                cur = Some((i, 3));
                break;
            }
        }
        if cur.is_none() {
            let mut best_diff = None;
            for (i, &c) in hand.iter().enumerate() {
                let l1 = *up1.last().unwrap();
                if l1 < c {
                    let diff = c - l1;
                    if best_diff.minim(diff) {
                        cur = Some((i, 0));
                    }
                }
                let l2 = *up2.last().unwrap();
                if l2 < c {
                    let diff = c - l2;
                    if best_diff.minim(diff) {
                        cur = Some((i, 1));
                    }
                }
                let l1 = *down1.last().unwrap();
                if l1 > c {
                    let diff = l1 - c;
                    if best_diff.minim(diff) {
                        cur = Some((i, 2));
                    }
                }
                let l2 = *down2.last().unwrap();
                if l2 > c {
                    let diff = l2 - c;
                    if best_diff.minim(diff) {
                        cur = Some((i, 3));
                    }
                }
            }
        }
        match cur {
            None => {
                break;
            }
            Some((pos, id)) => {
                let c = hand.remove(pos).unwrap();
                match id {
                    0 => {
                        up1.push(c);
                    }
                    1 => {
                        up2.push(c);
                    }
                    2 => {
                        down1.push(c);
                    }
                    3 => {
                        down2.push(c);
                    }
                    _ => unreachable!(),
                }
            }
        }
        if hand.len() <= 6 && !q.is_empty() {
            hand.push_back(q.pop_front().unwrap());
            hand.push_back(q.pop_front().unwrap());
        }
    }
    let hand: Vec<_> = hand.into();
    let q: Vec<_> = q.into();
    out_line!(up1);
    out_line!(up2);
    out_line!(down1);
    out_line!(down2);
    out_line!(hand);
    out_line!(q);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
