//{"name":"C. Суммы на подмассивах","group":"Codeforces - Educational Codeforces Round 145 (Rated for Div. 2)","url":"https://codeforces.com/contest/1809/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 2\n2 0\n2 2\n4 6\n","output":"1 -3 1\n-13 -42\n-13 42\n-3 -4 10 -2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSummiNaPodmassivakh"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut k = input.read_size();

    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if k >= i + 1 {
            ans.push(2);
            k -= i + 1;
        } else {
            ans.push(-2 * (i - k).into_i32() - 1);
            while ans.len() < n {
                ans.push(-1000);
            }
            break;
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
