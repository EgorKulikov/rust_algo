//{"name":"D. Два массива","group":"Codeforces - Codeforces Round #773 (Div. 1)","url":"https://codeforces.com/contest/1641/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4 2\n1 2 5\n4 3 1\n2 3 2\n4 5 3\n","output":"5\n"},{"input":"4 3\n1 2 3 5\n2 3 4 2\n3 4 5 3\n1 3 10 10\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDvaMassiva"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let mut v = input.read_unsigned_vec(m);
        v.sort();
        a.push((input.read_unsigned(), v));
    }

    a.sort();
    let mut viable = HashMap::new();
    let mut ans = None;
    viable.insert(Vec::new(), 0);
    for i in 1..n {
        let mut forb = Vec::new();
        loop {
            match viable.get(&forb) {
                None => {
                    break;
                }
                Some(&id) => {
                    let mut first = None;
                    for &j in &a[id].1 {
                        if a[i].1.contains(&j) {
                            first = Some(j);
                        } else {
                            let mut fc = forb.clone();
                            fc.push(j);
                            if !viable.contains_key(&fc) {
                                viable.insert(fc, i);
                            }
                        }
                    }
                    match first {
                        None => {
                            ans.minim(a[id].0 + a[i].0);
                            break;
                        }
                        Some(v) => {
                            forb.push(v);
                        }
                    }
                }
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
