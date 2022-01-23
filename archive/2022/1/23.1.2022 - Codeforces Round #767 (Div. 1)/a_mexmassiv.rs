//{"name":"A. MEX-массив","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\n1 0 2 0 3\n8\n2 2 3 4 0 1 2 0\n1\n1\n5\n0 1 2 3 4\n4\n0 1 1 0\n10\n0 0 2 1 1 1 0 0 1 1\n","output":"1\n4\n2\n5 1\n1\n0\n1\n5\n2\n2 2\n4\n3 2 2 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMEXMassiv"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut pos = vec![VecDeque::new(); n + 1];
    for (i, a) in a.into_iter().enumerate() {
        pos[a].push_back(i);
    }
    let mut c_pos = 0;
    let mut ans = Vec::with_capacity(n);
    while c_pos < n {
        let mut n_pos = c_pos;
        for i in 0..=n {
            let mut stop = false;
            loop {
                match pos[i].front() {
                    Some(&p) => {
                        if p < c_pos {
                            pos[i].pop_front();
                        } else {
                            n_pos.maxim(p);
                            break;
                        }
                    }
                    None => {
                        stop = true;
                        break;
                    }
                }
            }
            if stop {
                ans.push(i);
                break;
            }
        }
        c_pos = n_pos + 1;
    }
    out_line!(ans.len());
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
