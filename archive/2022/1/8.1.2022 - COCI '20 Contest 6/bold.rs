//{"name":"#1 - Bold","group":"DMOJ - COCI '20 Contest 6","url":"https://dmoj.ca/problem/coci20c6p1","interactive":false,"timeLimit":1000,"tests":[{"input":"4 4\n....\n.#..\n....\n....\n","output":"....\n.##.\n.##.\n....\n"},{"input":"7 7\n.......\n.####..\n.#...#.\n.#...#.\n.#...#.\n.####..\n.......\n","output":".......\n.#####.\n.######\n.##..##\n.##..##\n.######\n.#####.\n"},{"input":"9 7\n.......\n.####..\n.#...#.\n.#...#.\n.####..\n.#.....\n.#.....\n.#.....\n.......\n","output":".......\n.#####.\n.######\n.##..##\n.######\n.#####.\n.##....\n.##....\n.##....\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Bold"}}}

use algo_lib::collections::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut grid = input.read_table::<char>(n, m);

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if grid[(i, j)] == '#' {
                for k in i..i + 2 {
                    for l in j..j + 2 {
                        grid[(k, l)] = '#';
                    }
                }
            }
        }
    }
    output().print_table(&grid);
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
