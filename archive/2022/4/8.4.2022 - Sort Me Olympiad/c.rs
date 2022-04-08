//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s = input.read_table::<char>(n, 3);

    let mut ans = Arr3d::new(n + 1, 3, 3, -1);
    let mut rec = RecursiveFunction3::new(|f, row: usize, left: usize, right: usize| -> i32 {
        assert!(left < right);
        if ans[(row, left, right)] != -1 {
            return ans[(row, left, right)];
        }
        if row == n {
            ans[(row, left, right)] = 0;
            return 0;
        }
        let mut p1 = 3;
        let mut p2 = 3;
        for i in 0..3 {
            if s[(row, i)] == '#' {
                if p1 == 3 {
                    p1 = i;
                } else {
                    p2 = i;
                }
            }
        }
        if p2 == 3 {
            if left == p1 || right == p1 {
                ans[(row, left, right)] = f.call(row + 1, left, right);
            } else if p1 == 0 {
                ans[(row, left, right)] = f.call(row + 1, 0, right) + 1;
            } else if p1 == 2 {
                ans[(row, left, right)] = f.call(row + 1, left, 2) + 1;
            } else {
                ans[(row, left, right)] =
                    (f.call(row + 1, 0, 1) + 1).min(f.call(row + 1, 1, 2) + 1);
            }
            return ans[(row, left, right)];
        }
        if p1 == left {
            if p2 == right {
                ans[(row, left, right)] = f.call(row + 1, left, right);
            } else {
                ans[(row, left, right)] = f.call(row + 1, left, p2) + 1;
            }
        } else {
            if p2 == right {
                ans[(row, left, right)] = f.call(row + 1, p1, right) + 1;
            } else {
                ans[(row, left, right)] = f.call(row + 1, p1, p2) + 2;
            }
        }
        ans[(row, left, right)]
    });
    out_line!(rec.call(0, 0, 2));
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
