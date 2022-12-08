//{"name":"day8","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day8"}}}

// use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let mut c = Vec::new();
    while !input.is_exhausted() {
        c.push(input.read::<Str>());
        input.skip_whitespace();
    }

    let n = c.len();
    let m = c[0].len();
    /*let mut vis = Arr2d::new(n, m, false);
    for i in 0..n {
        let mut max = None;
        for j in 0..m {
            if max.maxim(c[i][j]) {
                vis[(i, j)] = true;
            }
        }
        let mut max = None;
        for j in (0..m).rev() {
            if max.maxim(c[i][j]) {
                vis[(i, j)] = true;
            }
        }
    }
    for j in 0..m {
        let mut max = None;
        for i in 0..n {
            if max.maxim(c[i][j]) {
                vis[(i, j)] = true;
            }
        }
        let mut max = None;
        for i in (0..n).rev() {
            if max.maxim(c[i][j]) {
                vis[(i, j)] = true;
            }
        }
    }*/
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            // if vis[(i, j)] {
            //     ans += 1;
            // }
            let mut res = 1;
            let mut cur = 0;
            for k in j + 1..m {
                cur += 1;
                if c[i][k] >= c[i][j] {
                    break;
                }
            }
            res *= cur;
            let mut cur = 0;
            for k in (0..j).rev() {
                cur += 1;
                if c[i][k] >= c[i][j] {
                    break;
                }
            }
            res *= cur;
            let mut cur = 0;
            for k in i + 1..n {
                cur += 1;
                if c[k][j] >= c[i][j] {
                    break;
                }
            }
            res *= cur;
            let mut cur = 0;
            for k in (0..i).rev() {
                cur += 1;
                if c[k][j] >= c[i][j] {
                    break;
                }
            }
            res *= cur;
            ans.maxim(res);
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
