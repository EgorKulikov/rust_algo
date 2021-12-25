//{"name":"G - Strongest Takahashi","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_g","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n##...\n.##..\n#.#..\n.....\n....#\n","output":"4\n"},{"input":"3\n...\n...\n...\n","output":"0\n"},{"input":"21\n.....................\n.....................\n...#.#...............\n....#.............#..\n...#.#...........#.#.\n..................#..\n.....................\n.....................\n.....................\n..........#.....#....\n......#..###.........\n........#####..#.....\n.......#######.......\n.....#..#####........\n.......#######.......\n......#########......\n.......#######..#....\n......#########......\n..#..###########.....\n.........###.........\n.........###.........\n","output":"19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GStrongestTakahashi"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::arr4d::Arr4d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read();
    let grid = input.read_table::<char>(n, n);

    let mut q = Arr2d::new(n + 1, n + 1, 0);
    for i in 0..n {
        for j in 0..n {
            q[(i + 1, j + 1)] = q[(i, j + 1)] + q[(i + 1, j)] - q[(i, j)];
            if grid[(i, j)] == '#' {
                q[(i + 1, j + 1)] += 1;
            }
        }
    }
    let mut ans = Arr4d::new(n, n + 1, n, n + 1, -1);
    let mut rec = RecursiveFunction4::new(
        |f, row_from: usize, row_to: usize, col_from: usize, col_to: usize| -> i32 {
            if row_from == row_to || col_from == col_to {
                0
            } else if ans[(row_from, row_to, col_from, col_to)] != -1 {
                ans[(row_from, row_to, col_from, col_to)]
            } else {
                let mut res = (row_to - row_from).max(col_to - col_from) as i32;
                for i in row_from..row_to {
                    if q[(i, col_from)] + q[(i + 1, col_to)]
                        == q[(i + 1, col_from)] + q[(i, col_to)]
                    {
                        res.minim(
                            f.call(row_from, i, col_from, col_to)
                                + f.call(i + 1, row_to, col_from, col_to),
                        );
                    }
                }
                for i in col_from..col_to {
                    if q[(row_from, i)] + q[(row_to, i + 1)]
                        == q[(row_from, i + 1)] + q[(row_to, i)]
                    {
                        res.minim(
                            f.call(row_from, row_to, col_from, i)
                                + f.call(row_from, row_to, i + 1, col_to),
                        );
                    }
                }
                ans[(row_from, row_to, col_from, col_to)] = res;
                res
            }
        },
    );
    out_line!(rec.call(0, n, 0, n));
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
