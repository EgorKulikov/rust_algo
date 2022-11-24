//{"name":"The Interstellar Problem","group":"CodeChef - STRV2022","url":"https://www.codechef.com/STRV2022/problems/STRPUZZLE","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n3 4\n1 1 0 0\n0 0 1 1\n1 0 1 0\n","output":"26\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TheInterstellarProblem"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut grid = input.read_table::<u32>(n, m);

    for i in 0..m {
        if grid[0][i] == 0 {
            grid.column_mut(i).for_each(|x| *x = 1 - *x);
        }
    }
    let mut ans = 0;
    for i in 0..n {
        let mut cur = 0;
        for j in 0..m {
            cur += grid[i][j];
        }
        cur.maxim(m.into_u32() - cur);
        ans += cur << (n - 1 - i);
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
    let test_type = TestType::MultiNumber;
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
