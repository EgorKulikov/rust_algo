//{"name":"C. Removing Smallest Multiples","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"http://codeforces.com/contest/1734/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6\n111111\n7\n1101001\n4\n0000\n4\n0010\n8\n10010101\n15\n110011100101100\n","output":"0\n11\n4\n4\n17\n60\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRemovingSmallestMultiples"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::primes::all_divisors;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let t: Str = input.read();

    let d = all_divisors(n + 1, true);
    let mut ans = 0;
    let mut forbidden = BitSet::new(n + 1);
    for i in 1..=n {
        if t[i - 1] == b'1' {
            for &j in &d[i] {
                forbidden.set(j, true);
            }
        } else {
            for &j in &d[i] {
                if !forbidden[j] {
                    ans += j;
                    break;
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
