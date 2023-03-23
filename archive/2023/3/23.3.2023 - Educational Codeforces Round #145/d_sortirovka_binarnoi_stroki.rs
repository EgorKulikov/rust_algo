//{"name":"D. Сортировка бинарной строки","group":"Codeforces - Educational Codeforces Round 145 (Rated for Div. 2)","url":"https://codeforces.com/contest/1809/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n100\n0\n0101\n00101101\n1001101\n11111\n","output":"1000000000001\n0\n1000000000000\n2000000000001\n2000000000002\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSortirovkaBinarnoiStroki"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let mut ones = s.iter().count_eq(&b'0');
    let mut ans = ones * 1000000000001;
    let mut zeroes = 0;
    for i in 0..s.len() {
        let c = s[i];
        if i + 1 < s.len() && c == b'1' && s[i + 1] == b'0' {
            ans.minim((zeroes + ones) * 1000000000001 - 1);
        }
        if c == b'1' {
            zeroes += 1;
        } else {
            ones -= 1;
        }
        ans.minim((zeroes + ones) * 1000000000001);
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
