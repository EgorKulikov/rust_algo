//{"name":"C. Равные частоты","group":"Codeforces - VK Cup 2022 - Отборочный раунд (Engine)","url":"https://codeforces.com/contest/1781/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\nhello\n10\ncodeforces\n5\neevee\n6\nappall\n","output":"1\nhelno\n2\ncodefofced\n1\neeeee\n0\nappall\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRavnieChastoti"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::cmp::Reverse;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let s: Str = input.read();

    let mut qty = vec![0; 26];
    for c in s.iter() {
        qty[c as usize - 'a' as usize] += 1;
    }
    let mut order = (0..26).collect_vec();
    order.sort_by_key(|&i| Reverse(qty[i]));
    let mut best = None;
    for i in 1..=26 {
        if n % i != 0 {
            continue;
        }
        let mut qq = qty.clone();
        let mut expected = vec![0; 26];
        for j in 0..i {
            expected[order[j]] = n / i;
        }
        let mut t = s.clone();
        let mut cur = 0;
        for j in 0..n {
            if qq[t[j] as usize - 'a' as usize] > expected[t[j] as usize - 'a' as usize] {
                cur += 1;
                for k in 0..26 {
                    if qq[k] < expected[k] {
                        qq[k] += 1;
                        qq[t[j] as usize - 'a' as usize] -= 1;
                        t[j] = (k + 'a' as usize) as u8;
                        break;
                    }
                }
            }
        }
        best.minim((cur, t));
    }
    let best = best.unwrap();
    out_line!(best.0);
    out_line!(best.1);
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
