//{"name":"E. Excel Sheet","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\naabb\nbaab\nbaab\n","output":"2\n"},{"input":"3 4\naaab\nbaab\nbaab\n","output":"4\n"},{"input":"4 5\nababa\nacaca\nccccb\ncbcbc\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EExcelSheet"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let t = input.read_table::<char>(n, m);

    let mut q = vec![0i64; 26];
    let mut ans = 0;
    for i in 0..m {
        for j in 0..i {
            q.fill(0);
            for k in 0..n {
                if t[(k, i)] == t[(k, j)] {
                    q[(t[(k, i)] as usize) - ('a' as usize)] += 1;
                }
            }
            for &k in &q {
                ans += k * (k - 1);
            }
        }
    }
    ans /= 2;
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
