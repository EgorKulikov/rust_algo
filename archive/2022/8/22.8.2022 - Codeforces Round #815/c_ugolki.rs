//{"name":"C. Уголки","group":"Codeforces - Codeforces Round #815 (Div. 2)","url":"https://codeforces.com/contest/1720/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 3\n101\n111\n011\n110\n3 4\n1110\n0111\n0111\n2 2\n00\n00\n2 2\n11\n11\n","output":"8\n9\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUgolki"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let c = input.read_table::<char>(n, m);

    let mut ans = c.iter().count_eq(&&'1');
    let mut min_inside = 3;
    for i in 0..n - 1 {
        for j in 0..m - 1 {
            let mut cur = 0;
            for k in 0..2 {
                for l in 0..2 {
                    if c[(i + k, j + l)] == '1' {
                        cur += 1;
                    }
                }
            }
            if cur <= 2 {
                min_inside = 1;
            } else if min_inside == 3 && cur == 3 {
                min_inside = 2;
            }
        }
    }
    ans -= min_inside - 1;
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
