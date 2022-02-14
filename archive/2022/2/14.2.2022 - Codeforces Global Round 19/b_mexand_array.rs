//{"name":"B. MEX and Array","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1 2\n3\n2 0 1\n4\n2 0 5 1\n5\n0 1 1 0 1\n","output":"4\n14\n26\n48\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMEXAndArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut ans = 0;
    for i in 0..=n {
        for j in 0..i {
            for &k in &a[j..i] {
                if k > 0 {
                    ans += 1;
                } else {
                    ans += 2;
                }
            }
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
