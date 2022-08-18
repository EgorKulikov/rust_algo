//{"name":"B. Also Try Minecraft","group":"Codeforces - Educational Codeforces Round 132 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1709/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"7 6\n10 8 9 6 8 12 7\n1 2\n1 7\n4 6\n7 1\n3 5\n4 2\n","output":"2\n10\n0\n7\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAlsoTryMinecraft"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_long_vec(n);

    let mut s_direct = Vec::with_capacity(n);
    s_direct.push(0);
    let mut cur = 0;
    for (&i, &j) in a.consecutive_iter() {
        cur += (i - j).max(0);
        s_direct.push(cur);
    }
    let mut s_reverse = Vec::with_capacity(n);
    s_reverse.push(0);
    cur = 0;
    for (&i, &j) in a.consecutive_iter() {
        cur += (j - i).max(0);
        s_reverse.push(cur);
    }

    for _ in 0..m {
        let s = input.read_usize() - 1;
        let t = input.read_usize() - 1;
        out_line!(if s < t {
            s_direct[t] - s_direct[s]
        } else {
            s_reverse[s] - s_reverse[t]
        });
    }
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
