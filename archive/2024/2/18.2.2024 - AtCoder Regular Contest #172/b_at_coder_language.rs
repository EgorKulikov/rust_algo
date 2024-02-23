//{"name":"B - AtCoder Language","group":"AtCoder - AtCoder Regular Contest 172","url":"https://atcoder.jp/contests/arc172/tasks/arc172_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3 2\n","output":"2\n"},{"input":"100 80 26\n","output":"496798269\n"},{"input":"100 1 26\n","output":"0\n"},{"input":"500000 172172 503746693\n","output":"869120\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAtCoderLanguage"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_int();
    let k = input.read_int();
    let l = input.read_int();

    if l <= n - k {
        out.print_line(0);
        return;
    }
    type Mod = ModIntF;
    let mut ans = Mod::one();
    for i in 0..n - k {
        ans *= Mod::new(l - i);
    }
    ans *= Mod::new(l - (n - k)).power(k);
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
