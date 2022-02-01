//{"name":"Breaking walls","group":"HackerEarth - January Circuits '22","url":"https://www.hackerearth.com/challenges/competitive/january-circuits-022/algorithm/break-the-wall-acf543a5/","interactive":false,"timeLimit":2000,"tests":[{"input":" 5 5\n *.#**\n *.#*.\n #####\n **#*.\n **#..\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BreakingWalls"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::dsu2d::DSU2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::Directions;
use algo_lib::{const_value_ref, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let grid = input.read_table::<char>(n, m);

    let mut dsu = DSU2d::new(n, m);
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] == '#' {
                continue;
            }
            if i + 1 < n && grid[(i + 1, j)] != '#' {
                dsu.join(i, j, i + 1, j);
            }
            if j + 1 < m && grid[(i, j + 1)] != '#' {
                dsu.join(i, j, i, j + 1);
            }
        }
    }
    let mut stars = Arr2d::new(n, m, 0);
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] == '*' {
                stars[dsu.get(i, j)] += 1;
            }
        }
    }
    const_value_ref!(
        Dirs,
        DIRS_INNER,
        [(isize, isize); 4],
        [(isize, isize)],
        [
            (0isize, 2isize),
            (2isize, 0isize),
            (1isize, 1isize),
            (-1isize, 1isize),
        ]
    );
    type D4 = Directions<Dirs>;
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] == '#' {
                continue;
            }
            let par = dsu.get(i, j);
            ans.maxim(stars[par]);
            for other in D4::iter(i, j, n, m) {
                if grid[other] != '#' {
                    let c_par = dsu.get(other.0, other.1);
                    if c_par != par {
                        ans.maxim(stars[par] + stars[c_par]);
                    }
                }
            }
        }
    }
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
