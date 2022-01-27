//{"name":"B. Отрезок и разбиение","group":"Codeforces - Codeforces Round #768 (Div. 1)","url":"https://codeforces.com/contest/1630/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 1\n1 2\n4 2\n1 2 2 2\n11 3\n5 5 5 1 5 5 1 5 5 5 1\n","output":"1 2\n1 2\n2 2\n1 3\n4 4\n5 5\n1 1\n2 2\n3 11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOtrezokIRazbienie"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut k = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut b = a.clone();
    b.sort_unstable();
    let mut x = 0;
    let mut y = 0;
    let mut dif = None;
    let delta = (n + k - 1) / 2;
    for i in 0..n - delta {
        if dif.minim(b[i + delta] - b[i]) {
            x = b[i];
            y = b[i + delta];
        }
    }
    out_line!(x, y);
    let mut start = 0;
    let mut balance = 0;
    for (i, a) in a.into_iter().enumerate() {
        if k == 1 {
            break;
        }
        if a >= x && a <= y {
            balance += 1;
        } else {
            balance -= 1;
        }
        if balance > 0 {
            out_line!(start + 1, i + 1);
            start = i + 1;
            balance = 0;
            k -= 1;
        }
    }
    out_line!(start + 1, n);
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
