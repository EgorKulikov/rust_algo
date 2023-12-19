//{"name":"D - Not Intersect","group":"AtCoder - AtCoder Grand Contest 065","url":"https://atcoder.jp/contests/agc065/tasks/agc065_d","interactive":false,"timeLimit":3000,"tests":[{"input":"4 2\n","output":"14\n"},{"input":"6 3\n","output":"295\n"},{"input":"2023 1217\n","output":"10811951\n"},{"input":"1234321 2345432\n","output":"789452255\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNotIntersect"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::{inverses, Combinations};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    if n >= 3 && 2 * n - 3 < m {
        out.print_line(0);
        return;
    }
    type Mod = ModInt7;
    let c = Combinations::<Mod>::new(2 * n);
    if n <= 3 {
        out.print_line(c.c(n * (n - 1) / 2, m));
        return;
    }
    let inv = inverses::<Mod>(n);
    let mut ans = Mod::zero();
    for i in 0..=(n - 3).min(m) {
        ans += c.c(n, m - i) * c.c(n - 3, i) * c.c(n + i - 1, i) * inv[i + 1];
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
                solve(&mut input, &mut output, i, &pre_calc);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
