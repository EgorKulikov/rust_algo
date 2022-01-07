//{"name":"B2. Basketbology (Counting)","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/B2","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n10 15 20\n28 16 18\n","output":"2\n"},{"input":"3\n10 15 20\n24 13 13\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B2BasketbologyCounting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::unsigned_big_int::UBigInt;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_unsigned_vec(n);
    let mut c = input.read_unsigned_vec(n);

    c.sort_unstable();
    let mut ans = UBigInt::one();
    let mut at = 0;
    let mut d = 0;
    for i in c {
        while at < n && x[at] <= i {
            at += 1;
            d += 1;
        }
        if d == 0 {
            out_line!(0);
            return;
        }
        ans *= d;
        d -= 1;
    }
    assert_eq!(d, 0);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
