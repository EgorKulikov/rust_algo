//{"name":"E. блокнот.exe","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/E","interactive":true,"timeLimit":1000,"tests":[{"input":"6\n? 1\n\n? 9\n\n? 16\n\n! 32\n","output":"\n0\n\n4\n\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBloknotexe"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    let mut left = 2 * n - 1;
    let mut right = 2000 * n + n - 1;
    while left < right {
        let mid = (left + right) >> 1;
        out_line!('?', mid);
        if input.read_int() == 1 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let mut ans = left;
    for i in 2..=n {
        out_line!('?', left / i);
        if input.read_usize() == i {
            ans.minim(left / i * i);
        }
    }
    out_line!('!', ans);
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
