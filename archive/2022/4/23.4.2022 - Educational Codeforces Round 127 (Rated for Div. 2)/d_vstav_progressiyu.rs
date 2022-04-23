//{"name":"D. Вставь прогрессию","group":"Codeforces - Educational Codeforces Round 127 (Rated for Div. 2)","url":"https://codeforces.com/contest/1671/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 5\n10\n3 8\n7 2 10\n10 2\n6 1 5 7 3 3 9 10 10 1\n4 10\n1 3 1 2\n","output":"9\n15\n31\n13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVstavProgressiyu"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_long();
    let a = input.read_long_vec(n);

    let mut ans = 0;
    let mut add_min = a[0] - 1;
    let mut add_max = (a[0] - x).abs();
    let mut add_both = x - 1 + (a[0] - x).abs();
    for (&y, &z) in a.consecutive_iter() {
        ans += (z - y).abs();
        add_min.minim((y - 1).abs() + (z - 1).abs() - (y - z).abs());
        add_max.minim((y - x).abs() + (x - z).abs() - (y - z).abs());
        add_both.minim((y - 1).abs() + (x - 1) + (z - x).abs() - (y - z).abs());
        add_both.minim((y - x).abs() + (x - 1) + (z - 1).abs() - (y - z.abs()));
    }
    add_min.minim((a[n - 1] - 1).abs());
    add_max.minim((a[n - 1] - x).abs());
    add_both.minim((a[n - 1] - x).abs() + x - 1);
    ans += (add_min + add_max).min(add_both);
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
