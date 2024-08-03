//{"name":"E - Xor Sigma Problem","group":"AtCoder - Toyota Programming Contest 2024#8（AtCoder Beginner Contest 365）","url":"https://atcoder.jp/contests/abc365/tasks/abc365_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3 2\n","output":"3\n"},{"input":"7\n2 5 6 5 2 1 7\n","output":"83\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EXorSigmaProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_vec::<u64>(n);

    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] ^ a[i]);
    }
    let mut ans = 0;
    for i in 0..30 {
        let mut ones = 0u64;
        let mut zeroes = 0u64;
        for &j in &s {
            if j.is_set(i) {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }
        ans += (ones * zeroes) << i;
    }
    for i in a {
        ans -= i;
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
}
//END MAIN
