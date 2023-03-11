//{"name":"Task","group":"HackerEarth","url":"https://www.hackerearth.com/challenges/competitive/dsa-coding-contest-march-23/problems/0eddfc6f20d34c2282089ab6462c8596/","interactive":false,"timeLimit":1500,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_table(n, m);

    let rs = (0..n).map(|i| a.row(i).sum::<i64>()).collect_vec();
    let cs = (0..m).map(|j| a.column(j).sum::<i64>()).collect_vec();
    let mut ans = None;
    let mut row = 0;
    let mut col = 0;
    for i in 0..n {
        for j in 0..m {
            let cur = rs[i] + cs[j] - 2 * a[i][j];
            if ans.maxim(cur) {
                row = i;
                col = j;
            }
        }
    }
    out_line!(row + 1, col + 1);
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
