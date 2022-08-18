//{"name":"F - Monochromatic Path","group":"AtCoder - freee Programming Contest 2022（AtCoder Beginner Contest 264）","url":"https://atcoder.jp/contests/abc264/tasks/abc264_f","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n4 3 5\n2 6 7 4\n0100\n1011\n1010\n","output":"9\n"},{"input":"15 20\n29 27 79 27 30 4 93 89 44 88 70 75 96 3 78\n39 97 12 53 62 32 38 84 49 93 53 26 13 25 2 76 32 42 34 18\n01011100110000001111\n10101111100010011000\n11011000011010001010\n00010100011111010100\n11111001101010001011\n01111001100101011100\n10010000001110101110\n01001011100100101000\n11001000100101011000\n01110000111011100101\n00111110111110011111\n10101111111011101101\n11000011000111111001\n00011101011110001101\n01010000000001000000\n","output":"125\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMonochromaticPath"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::arr4d::Arr4d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let h = input.read_usize();
    let w = input.read_usize();
    let r = input.read_long_vec(h);
    let c = input.read_long_vec(w);
    let a = input.read_table::<char>(h, w);

    let mut ans = Arr4d::new(h, w, 2, 2, std::i64::MAX);
    ans[(0, 0, 0, 0)] = 0;
    ans[(0, 0, 0, 1)] = c[0];
    ans[(0, 0, 1, 0)] = r[0];
    ans[(0, 0, 1, 1)] = r[0] + c[0];
    for i in 0..h {
        for j in 0..w {
            for k in 0..2 {
                for l in 0..2 {
                    if i + 1 < h {
                        if (a[(i, j)] != a[(i + 1, j)]) == (k != 0) {
                            let cand = ans[(i, j, k, l)];
                            ans[(i + 1, j, 0, l)].minim(cand);
                        } else {
                            let cand = ans[(i, j, k, l)] + r[i + 1];
                            ans[(i + 1, j, 1, l)].minim(cand);
                        }
                    }
                    if j + 1 < w {
                        if (a[(i, j)] != a[(i, j + 1)]) == (l != 0) {
                            let cand = ans[(i, j, k, l)];
                            ans[(i, j + 1, k, 0)].minim(cand);
                        } else {
                            let cand = ans[(i, j, k, l)] + c[j + 1];
                            ans[(i, j + 1, k, 1)].minim(cand);
                        }
                    }
                }
            }
        }
    }
    let mut res = std::i64::MAX;
    for i in 0..2 {
        for j in 0..2 {
            res.minim(ans[(h - 1, w - 1, i, j)]);
        }
    }
    out_line!(res);
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
