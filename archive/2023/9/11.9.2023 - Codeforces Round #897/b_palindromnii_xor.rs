//{"name":"B. Палиндромный XOR","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n6\n101011\n5\n00000\n9\n100100011\n3\n100\n1\n1\n","output":"0010100\n111111\n0011111100\n0110\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPalindromniiXOR"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut from = 0;
    let mut plus2 = 0;
    let plus1 = n % 2 == 1;
    for i in 0..n / 2 {
        if s[i] == s[n - 1 - i] {
            plus2 += 1;
        } else {
            from += 1;
        }
    }
    let mut ans = Str::from(vec![b'0'; n + 1]);
    if plus1 {
        for i in from..=from + 2 * plus2 + 1 {
            ans[i] = b'1';
        }
    } else {
        for i in (from..=from + 2 * plus2).step_by(2) {
            ans[i] = b'1';
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
