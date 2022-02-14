//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::fwht::FWHT;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    const LEN: usize = 1 << 14;
    let mut qty = [0i64; LEN];
    for &i in &a {
        qty[i] += 1;
    }
    let mut ans = 5 * qty[0] * (qty[0] - 1) * (qty[0] - 2) / 6;
    for &i in qty.iter().skip(1) {
        ans += 2 * qty[0] * i * (i - 1);
    }
    qty.fwht(false);
    for v in &mut qty {
        *v *= *v;
    }
    qty.fwht(true);
    for i in a {
        ans += qty[i];
    }
    assert_eq!(ans % 6, 0);
    ans /= 6;
    out_line!(ans);
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
