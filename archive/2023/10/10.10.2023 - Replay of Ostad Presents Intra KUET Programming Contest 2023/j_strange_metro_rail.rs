//{"name":"J. Strange Metro Rail","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/J","interactive":false,"timeLimit":1000,"tests":[{"input":"6 10\n","output":"31\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JStrangeMetroRail"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let l = input.read_long();
    let r = input.read_long();

    let d = r.divisors();
    let mut qty = Vec::with_capacity(d.len());
    let mut ans = 0i128;
    for &x in d.iter().rev() {
        let mut cur = r / x - (l - 1) / x;
        for (&y, &z) in d.iter().rev().zip(qty.iter()) {
            if y % x == 0 {
                cur -= z;
            }
        }
        qty.push(cur);
        ans += (cur as i128) * ((r / x) as i128);
    }
    out.print_line(ans % 1000000007);
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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
