//{"name":"G - Black and White Stones","group":"AtCoder - Tokio Marine & Nichido Fire Insurance Programming Contest 2022（AtCoder Beginner Contest 256)","url":"https://atcoder.jp/contests/abc256/tasks/abc256_g","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n","output":"10\n"},{"input":"299792458 3141\n","output":"138897974\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GBlackAndWhiteStones"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let d = input.read_usize();

    type Mod = ModIntF;
    let mut ans = Mod::new(2);
    let c: Combinations<Mod> = Combinations::new(d);
    for i in 1..=d {
        let mut matrix = Matrix::zero(2, 2);
        if i > 1 {
            matrix[(0, 0)] = c.c(d - 1, i - 2);
        }
        matrix[(0, 1)] = c.c(d - 1, i - 1);
        matrix[(1, 0)] = c.c(d - 1, i - 1);
        if i != d {
            matrix[(1, 1)] = c.c(d - 1, i);
        }
        let res = matrix.power(n);
        ans += res[(0, 0)] + res[(1, 1)];
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
