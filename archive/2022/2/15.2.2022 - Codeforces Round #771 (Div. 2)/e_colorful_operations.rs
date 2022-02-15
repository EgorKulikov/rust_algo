//{"name":"E. Colorful Operations","group":"Codeforces - Codeforces Round #771 (Div. 2)","url":"https://codeforces.com/contest/1638/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5 8\nColor 2 4 2\nAdd 2 2\nQuery 3\nColor 4 5 3\nColor 2 2 3\nAdd 3 3\nQuery 2\nQuery 5\n","output":"2\n5\n3\n"},{"input":"2 7\nAdd 1 7\nQuery 1\nAdd 2 4\nQuery 2\nColor 1 1 1\nAdd 1 1\nQuery 2\n","output":"7\n7\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EColorfulOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();

    let mut color = vec![None; 4 * n];
    let mut add = vec![0i64; 4 * n];
    let mut balance = vec![0; n];
    color[0] = Some(0);

    for _ in 0..q {
        let t = input.read_string();
        match t.as_str() {
            "Color" => {
                let from = input.read_usize() - 1;
                let to = input.read_usize();
                let c = input.read_usize() - 1;
                let mut update = RecursiveFunction3::new(|f, root, left, right| {
                    if left >= to || right <= from {
                        return;
                    }
                    let mid = (left + right) >> 1;
                    match color[root] {
                        None => {
                            f.call(2 * root + 1, left, mid);
                            f.call(2 * root + 2, mid, right);
                            if left >= from && right <= to {
                                color[2 * root + 1] = None;
                                color[2 * root + 2] = None;
                                color[root] = Some(c);
                            }
                        }
                        Some(col) => {
                            if col == c {
                                return;
                            }
                            if left >= from && right <= to {
                                add[root] += balance[col] - balance[c];
                                color[root] = Some(c);
                            } else {
                                color[2 * root + 1] = Some(col);
                                color[2 * root + 2] = Some(col);
                                color[root] = None;
                                f.call(2 * root + 1, left, mid);
                                f.call(2 * root + 2, mid, right);
                            }
                        }
                    }
                });
                update.call(0, 0, n);
            }
            "Add" => {
                let c = input.read_usize() - 1;
                let x = input.read_long();
                balance[c] += x;
            }
            "Query" => {
                let at = input.read_usize() - 1;
                let mut query = RecursiveFunction3::new(|f, root, left, right| {
                    let mut res = 0i64;
                    if let Some(col) = color[root] {
                        res += balance[col];
                    }
                    res += add[root];
                    if left + 1 != right {
                        let mid = (left + right) >> 1;
                        if at < mid {
                            res += f.call(2 * root + 1, left, mid);
                        } else {
                            res += f.call(2 * root + 2, mid, right);
                        }
                    }
                    res
                });
                out_line!(query.call(0, 0, n));
            }
            _ => unreachable!(),
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
