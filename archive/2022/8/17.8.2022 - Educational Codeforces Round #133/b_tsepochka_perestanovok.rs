//{"name":"B. Цепочка перестановок","group":"Codeforces - Educational Codeforces Round 133 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1716/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2\n3\n","output":"2\n1 2\n2 1\n3\n1 2 3\n3 2 1\n3 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTsepochkaPerestanovok"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    let mut ans = (1..=n).collect_vec();
    out_line!(n);
    out_line!(ans);
    for i in 1..n {
        ans.swap(0, i);
        out_line!(ans);
    }
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
