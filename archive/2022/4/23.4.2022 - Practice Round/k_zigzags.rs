//{"name":"K. Zigzags","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/K","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n14 1 10 8 6 8 10 13 10 7\n","output":"4\n5 6 7 10\n"},{"input":"2\n-5 -5\n","output":"1\n2\n"},{"input":"5\n2 3 -5 8 -13\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KZigzags"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    let mut ans = Vec::new();
    let mut prev = a[0];
    for (i, (&a, &b)) in a.consecutive_iter().skip(1).enumerate() {
        if a == prev || a == b || prev < a && a < b || prev > a && a > b {
            ans.push(i + 2);
        } else {
            prev = a;
        }
    }
    if n > 1 && a[n - 1] == prev {
        ans.push(n);
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
