//{"name":"C. LR-остатки","group":"Codeforces - Codeforces Round 927 (Div. 3)","url":"https://codeforces.com/contest/1932/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 6\n3 1 4 2\nLRRL\n5 1\n1 1 1 1 1\nLLLLL\n6 8\n1 2 3 4 5 6\nRLLLRR\n1 10000\n10000\nR\n","output":"0 2 4 1\n0 0 0 0 0\n0 0 0 4 4 4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLROstatki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_long();
    let a = input.read_long_vec(n);
    let s = input.read_str();

    let mut left = 0;
    let mut right = n;
    for c in s.iter() {
        match c {
            b'L' => left += 1,
            b'R' => right -= 1,
            _ => unreachable!(),
        }
    }
    let mut ans = 1;
    let mut b = Vec::with_capacity(n);
    for c in s.iter().rev() {
        match c {
            b'L' => {
                left -= 1;
                ans *= a[left];
                ans %= m;
            }
            b'R' => {
                ans *= a[right];
                ans %= m;
                right += 1;
            }
            _ => unreachable!(),
        }
        b.push(ans);
    }
    b.reverse();
    out.print_line(b);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
