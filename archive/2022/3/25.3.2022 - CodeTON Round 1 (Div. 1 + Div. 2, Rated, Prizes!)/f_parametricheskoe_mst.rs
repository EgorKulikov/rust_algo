//{"name":"F. Параметрическое MST","group":"Codeforces - CodeTON Round 1 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1656/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n1 0\n2\n-1 1\n3\n1 -1 -2\n3\n3 -1 -2\n4\n1 2 3 -4\n","output":"INF\n-1\nINF\n-6\n-18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FParametricheskoeMST"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_long_vec(n);

    a.sort_unstable();
    let s: i64 = a.iter().sum();
    let min = a[0];
    let max = a[n - 1];
    if (n.into_i64() - 2) * max + s < 0 || (n.into_i64() - 2) * min + s > 0 {
        out_line!("INF");
        return;
    }
    let mut k = max + min;
    let mut b = max * min;
    for &aa in &a[1..n - 1] {
        k += aa + a[0];
        b += aa * a[0];
    }
    let mut ans = None;
    if k == 0 {
        ans.maxim(b);
    }
    for &aa in &a[1..n - 1] {
        ans.maxim(b - k * aa);
        k -= aa + a[0];
        k += aa + a[n - 1];
        b -= aa * a[0];
        b += aa * a[n - 1];
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
