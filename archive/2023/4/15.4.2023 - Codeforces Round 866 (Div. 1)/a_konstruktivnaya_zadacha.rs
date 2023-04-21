//{"name":"A. Конструктивная задача","group":"Codeforces - Codeforces Round 866 (Div. 1)","url":"https://codeforces.com/contest/1819/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 1\n4\n0 2 2 0\n4\n3 2 0 2\n1\n0\n","output":"Yes\nYes\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKonstruktivnayaZadacha"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let set = a.iter().copied().collect::<HashSet<_>>();
    let mut mex = 0;
    for i in 0.. {
        if !set.contains(&i) {
            mex = i;
            break;
        }
    }
    if mex == n {
        out_line!(false);
        return;
    }
    let mut left = None;
    let mut right = None;
    for (i, &a) in a.iter().enumerate() {
        if a == mex + 1 {
            if left.is_none() {
                left = Some(i);
            }
            right = Some(i);
        }
    }
    if left.is_none() {
        out_line!(true);
        return;
    }
    let mut other = HashSet::new();
    for i in 0..left.unwrap() {
        other.insert(a[i]);
    }
    for i in right.unwrap() + 1..n {
        other.insert(a[i]);
    }
    for i in 0..mex {
        if !other.contains(&i) {
            out_line!(false);
            return;
        }
    }
    out_line!(true);
}

pub(crate) fn run(mut input: Input) -> bool {
    set_bool_output(BoolOutput::YesNo);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
