//{"name":"B. Марк-уборщик","group":"Codeforces - Codeforces Round #807 (Div. 2)","url":"https://codeforces.com/contest/1705/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n3\n2 0 0\n5\n0 2 0 2 0\n6\n2 0 3 0 4 6\n4\n0 0 0 10\n","output":"3\n5\n11\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMarkUborshchik"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long_vec(n);

    let mut from_start = true;
    let mut ans = 0;
    for i in a.into_iter().take(n - 1) {
        if i == 0 {
            if !from_start {
                ans += 1;
            }
        } else {
            from_start = false;
            ans += i;
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
