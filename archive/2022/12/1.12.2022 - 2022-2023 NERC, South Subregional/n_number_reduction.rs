//{"name":"N. Number Reduction","group":"Codeforces - 2022-2023 ICPC, NERC, Южный четвертьфинал (онлайн-трансляция, правила ICPC, командное соревнование)","url":"https://codeforces.com/contest/1765/problem/N","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n10000\n4\n1337\n0\n987654321\n6\n66837494128\n5\n7808652\n3\n","output":"1\n1337\n321\n344128\n7052\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NNumberReduction"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let x: Str = input.read();
    let k = input.read_size();

    let mut cnt = vec![VecDeque::new(); 10];
    for (i, c) in x.iter().take(k).enumerate() {
        cnt[c.into_usize() - b'0'.into_usize()].push_back(i);
    }
    let mut ans = Str::new();
    for (i, c) in x.iter().enumerate().skip(k) {
        cnt[c.into_usize() - b'0'.into_usize()].push_back(i);
        let start = if ans.is_empty() { 1 } else { 0 };
        for i in start..10 {
            if !cnt[i].is_empty() {
                ans.push(i as u8 + b'0');
                let f = cnt[i].pop_front().unwrap();
                for j in 0..10 {
                    while !cnt[j].is_empty() && *cnt[j].front().unwrap() < f {
                        cnt[j].pop_front();
                    }
                }
                break;
            }
        }
    }
    out_line!(ans);
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
