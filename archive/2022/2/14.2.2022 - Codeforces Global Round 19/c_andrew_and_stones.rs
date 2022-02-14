//{"name":"C. Andrew and Stones","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n1 2 2 3 6\n3\n1 3 1\n3\n1 2 1\n4\n3 1 1 2\n","output":"4\n-1\n1\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAndrewAndStones"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long_vec(n);

    if a[1..n - 1].iter().max().unwrap() == &1 {
        out_line!("-1");
        return;
    }
    if n == 3 && a[1] % 2 == 1 {
        out_line!("-1");
        return;
    }
    let mut ans = 0;
    for &i in &a[1..n - 1] {
        ans += i + i % 2;
    }
    out_line!(ans / 2);
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
