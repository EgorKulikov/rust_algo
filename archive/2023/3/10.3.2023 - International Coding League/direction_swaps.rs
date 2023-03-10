//{"name":"Direction Swaps","group":"CodeChef - INCL2023","url":"https://www.codechef.com/INCL2023/problems/ICL_DIRSWP","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\nRLR\nUDL\nDUR\n","output":"2\n"},{"input":"6 6\nDDLURU\nRDLUDL\nRDUDLD\nURLLRR\nDUDDUL\nRRURDR\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DirectionSwaps"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let grid = input.read_char_table(n, m);

    let mut ans = 0;
    for i in 0..n {
        let mut right = 0;
        for &c in grid.row(i) {
            match c {
                'R' => right += 1,
                'L' => ans += right,
                _ => right = 0,
            }
        }
    }
    for i in 0..m {
        let mut down = 0;
        for &c in grid.column(i) {
            match c {
                'D' => down += 1,
                'U' => ans += down,
                _ => down = 0,
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
