//{"name":"D1. Anagram Pairs #1","group":"CPython.uz - CPython Beginner Contest #37","url":"https://cpython.uz/competitions/contests/contest/312/problem/D1","interactive":false,"timeLimit":1000,"tests":[{"input":"abba\n","output":"4\n"},{"input":"abcd\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1AnagramPairs1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    let mut ans = 0i64;
    let mut map = HashMap::<Vec<i32>, i64>::new();
    for i in 0..s.len() {
        let mut cur = vec![0i32; 26];
        for c in s.iter().skip(i) {
            cur[(c - b'a') as usize] += 1;
            let val = *map.get(&cur).unwrap_or(&0);
            ans += val;
            map.insert(cur.clone(), val + 1);
        }
    }
    out.print_line(ans);
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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
