//{"name":"C. Восстанови ПСП","group":"Codeforces - Educational Codeforces Round 132 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1709/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n(?))\n??????\n()\n??\n?(?)()?)\n","output":"YES\nNO\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVosstanoviPSP"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let n = s.len();
    if n % 2 != 0 {
        out_line!(false);
        return;
    }
    let mut d_min = Vec::with_capacity(n + 1);
    let mut d_max = Vec::with_capacity(n + 1);
    let mut r_min = Vec::with_capacity(n + 1);
    let mut r_max = Vec::with_capacity(n + 1);
    for (iter, min, max, open, close) in [
        (s.iter().collect_vec(), &mut d_min, &mut d_max, b'(', b')'),
        (
            s.iter().rev().collect_vec(),
            &mut r_min,
            &mut r_max,
            b')',
            b'(',
        ),
    ] {
        min.push(0);
        max.push(0);
        let mut c_min = 0;
        let mut c_max = 0;
        for c in iter {
            if c == open {
                c_min += 1;
                c_max += 1;
            } else if c == close {
                c_min -= 1;
                c_max -= 1;
            } else {
                c_min -= 1;
                c_max += 1;
            }
            if c_min < 0 {
                c_min += 2;
            }
            min.push(c_min);
            max.push(c_max);
        }
    }
    r_min.reverse();
    r_max.reverse();
    for i in 0..=n {
        let min = d_min[i].max(r_min[i]);
        let max = d_max[i].min(r_max[i]);
        if min != max {
            out_line!(false);
            return;
        }
    }
    out_line!(true);
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
