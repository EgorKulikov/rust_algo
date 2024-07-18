//{"name":"E. Geometric Progression Subsequence","group":"Codeforces - Coding Challenge Alpha V - by AlgoRave","url":"https://codeforces.com/gym/105005/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1 3 9\n","output":"2\n"},{"input":"6\n1 2 1 4 16 1\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EGeometricProgressionSubsequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut ans = 0;
    for i in 0..n {
        let mut seen = 0;
        let mut want = 0;
        for j in i..n {
            if want.is_set(a[j]) {
                ans += n - j;
                break;
            }
            let x = a[j] * a[j];
            for k in 1..=30 {
                if seen.is_set(k) && x % k == 0 {
                    let y = x / k;
                    if y <= 30 {
                        want.set_bit(y);
                    }
                }
            }
            seen.set_bit(a[j]);
        }
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
    //    tester::stress_test();
}
//END MAIN
