//{"name":"D. Friends and the Restaurant","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6\n8 3 9 2 4 5\n5 3 1 4 5 10\n4\n1 2 3 4\n1 1 2 2\n3\n2 3 7\n1 3 10\n6\n2 3 6 9 5 7\n3 2 7 10 6 10\n6\n5 4 2 1 8 100\n1 1 1 1 1 200\n6\n1 4 1 2 4 2\n1 3 3 2 3 4\n","output":"2\n0\n1\n3\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DFriendsAndTheRestaurant"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_int_vec(n);
    let y = input.read_int_vec(n);

    let mut f = x.into_iter().zip(y.into_iter()).collect_vec();
    f.sort_by_key(|&(x, y)| x - y);
    let mut start = 0;
    let mut end = n - 1;
    let mut ans = 0;
    while start < end {
        let (x1, y1) = f[start];
        let (x2, y2) = f[end];
        if x1 + x2 <= y1 + y2 {
            ans += 1;
            start += 1;
            end -= 1;
        } else {
            end -= 1;
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
