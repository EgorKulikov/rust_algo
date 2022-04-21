//{"name":"A. Make it Increasing","group":"Codeforces - Codeforces Round #783 (Div. 1)","url":"https://codeforces.com/contest/1667/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2 3 4 5\n","output":"4\n"},{"input":"7\n1 2 1 2 1 2 1\n","output":"10\n"},{"input":"8\n1 8 2 7 3 6 4 5\n","output":"16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMakeItIncreasing"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long_vec(n);

    fn moves(it: impl Iterator<Item = i64>) -> i64 {
        let mut ans = 0;
        let mut cur = 0;
        for i in it {
            let times = cur / i + 1;
            ans += times;
            cur = times * i;
        }
        ans
    }

    let mut ans = None;
    for i in 0..n {
        ans.minim(moves(a.iter().skip(i + 1).cloned()) + moves(a.iter().take(i).rev().cloned()));
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
