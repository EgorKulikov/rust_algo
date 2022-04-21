//{"name":"F. Съедая конфеты","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n10 20 10\n6\n2 1 4 2 4 1\n5\n1 2 4 8 16\n9\n7 3 20 5 15 1 11 8 10\n","output":"2\n6\n0\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSedayaKonfeti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let w = input.read_int_vec(n);

    let mut i = 0;
    let mut j = n;
    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;
    while i <= j {
        if left == right {
            ans = i + n - j;
        }
        if left <= right {
            left += w[i];
            i += 1;
        } else {
            j -= 1;
            right += w[j];
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
