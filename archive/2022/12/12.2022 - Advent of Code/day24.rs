//{"name":"day24","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day24"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let mut map = Vec::new();
    while !input.is_exhausted() {
        map.push(input.read::<Str>());
        input.skip_whitespace();
    }

    let n = map.len();
    let m = map[0].len();
    let mut left = Arr2d::generate(n, m, |i, j| map[i][j] == b'<');
    let mut right = Arr2d::generate(n, m, |i, j| map[i][j] == b'>');
    let mut up = Arr2d::generate(n, m, |i, j| map[i][j] == b'^');
    let mut down = Arr2d::generate(n, m, |i, j| map[i][j] == b'v');
    let mut can_be_at = Arr2d::new(n, m, false);
    let mut next = Arr2d::new(n, m, false);
    can_be_at[(0, 1)] = true;

    let mut reached_goal = false;
    let mut reached_start = false;
    for i in 0.. {
        // if can_be_at[(n - 1, m - 2)] {
        //     out_line!(i);
        //     return;
        // }
        if can_be_at[(n - 1, m - 2)] && reached_start {
            out_line!(i);
            return;
        }
        if !reached_start && can_be_at[(0, 1)] && reached_goal {
            reached_start = true;
            can_be_at.fill(false);
            can_be_at[(0, 1)] = true;
        }
        if !reached_goal && can_be_at[(n - 1, m - 2)] {
            reached_goal = true;
            can_be_at.fill(false);
            can_be_at[(n - 1, m - 2)] = true;
        }
        next.fill(false);
        for i in 0..n {
            for j in 0..m {
                if !can_be_at[(i, j)]
                    || left[(i, j)]
                    || right[(i, j)]
                    || up[(i, j)]
                    || down[(i, j)]
                    || map[i][j] == b'#'
                {
                    continue;
                }
                next[(i, j)] = true;
                for (ni, nj) in D4::iter(i, j, n, m) {
                    next[(ni, nj)] = true;
                }
            }
        }
        swap(&mut can_be_at, &mut next);
        for i in 0..n {
            let last = left[(i, 1)];
            for j in 2..m - 1 {
                left[(i, j - 1)] = left[(i, j)];
            }
            left[(i, m - 2)] = last;
            let last = right[(i, m - 2)];
            for j in (2..m - 1).rev() {
                right[(i, j)] = right[(i, j - 1)];
            }
            right[(i, 1)] = last;
        }
        for j in 0..m {
            let last = up[(1, j)];
            for i in 2..n - 1 {
                up[(i - 1, j)] = up[(i, j)];
            }
            up[(n - 2, j)] = last;
            let last = down[(n - 2, j)];
            for i in (2..n - 1).rev() {
                down[(i, j)] = down[(i - 1, j)];
            }
            down[(1, j)] = last;
        }
    }
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
