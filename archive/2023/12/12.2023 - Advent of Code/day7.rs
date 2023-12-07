//{"name":"day7","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day7"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::collections::slice_ext::reversed::ReversedSlice;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut hands = Vec::new();
    while !input.is_exhausted() {
        hands.push((input.read_str(), input.read_long()));
    }

    {
        // part 1
        let ranks = b"23456789TJQKA";
        let hands = hands
            .iter()
            .map(|(s, bet)| {
                let mut sz = Vec::new();
                for i in 0..5 {
                    let mut done = false;
                    for j in 0..i {
                        if s[i] == s[j] {
                            done = true;
                            break;
                        }
                    }
                    if done {
                        continue;
                    }
                    let mut qty = 1;
                    for j in i + 1..5 {
                        if s[i] == s[j] {
                            qty += 1;
                        }
                    }
                    sz.push(qty);
                }
                sz.sort();
                sz.reverse();
                let mut hand = Vec::new();
                for c in s.iter() {
                    hand.push(ranks.iter().find_eq(&c).unwrap());
                }
                (sz, hand, *bet)
            })
            .collect_vec()
            .sorted();
        let mut ans = 0;
        for (i, (_, _, bet)) in hands.iter().enumerate() {
            ans += (i as i64 + 1) * (*bet);
        }
        out.print_line(ans);
    }
    {
        // part 2
        let ranks = b"J23456789TQKA";
        let hands = hands
            .iter()
            .map(|(s, bet)| {
                let mut sz = Vec::new();
                let mut j = 0;
                for i in 0..5 {
                    let mut done = false;
                    for j in 0..i {
                        if s[i] == s[j] {
                            done = true;
                            break;
                        }
                    }
                    if done {
                        continue;
                    }
                    let mut qty = 1;
                    for j in i + 1..5 {
                        if s[i] == s[j] {
                            qty += 1;
                        }
                    }
                    if s[i] == b'J' {
                        j = qty;
                    } else {
                        sz.push(qty);
                    }
                }
                sz.sort();
                sz.reverse();
                if sz.is_empty() {
                    sz.push(j);
                } else {
                    sz[0] += j;
                }
                let mut hand = Vec::new();
                for c in s.iter() {
                    hand.push(ranks.iter().find_eq(&c).unwrap());
                }
                (sz, hand, *bet)
            })
            .collect_vec()
            .sorted();
        let mut ans = 0;
        for (i, (_, _, bet)) in hands.iter().enumerate() {
            ans += (i as i64 + 1) * (*bet);
        }
        out.print_line(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
