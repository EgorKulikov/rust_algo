//{"name":"E. Уравнивание людей","group":"Codeforces - Codeforces Round #843 (Div. 2)","url":"https://codeforces.com/contest/1775/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n1 2 -3\n5\n1 0 0 -1 -1\n6\n2 -4 3 -5 4 1\n5\n1 -1 1 -1 1\n7\n0 0 0 0 0 0 0\n","output":"3\n2\n6\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EUravnivanieLyudei"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut b = Vec::new();
    let mut s = 1;
    b.push(0);
    for i in a {
        if i == 0 {
            continue;
        }
        if i * s > 0 {
            *b.last_mut().unwrap() += i.abs();
        } else {
            b.push(i.abs());
            s = -s;
        }
    }

    let mut v = VecDeque::new();
    for i in b {
        v.push_front(i);
        while v.len() >= 3 && v[1] <= v[0] && v[1] <= v[2] {
            let res = v[0] + v[2] - v[1];
            v.pop_front();
            v.pop_front();
            v.pop_front();
            v.push_front(res);
        }
    }
    out_line!(v.into_iter().max());
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
