//{"name":"E. Миша и раскраски","group":"Codeforces - Codeforces Round #815 (Div. 2)","url":"https://codeforces.com/contest/1720/problem/E","interactive":false,"timeLimit":3500,"tests":[{"input":"3 4\n1 1 1\n1 1 2\n3 4 5\n","output":"1\n"},{"input":"3 2\n2 1 3\n2 1 1\n3 1 2\n","output":"2\n"},{"input":"3 3\n1 1 1\n1 1 2\n2 2 2\n","output":"1\n"},{"input":"3 2\n1 1 1\n1 2 1\n2 2 2\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMishaIRaskraski"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let kk = input.read_usize();
    let a = input.read_table::<usize>(n, n);

    let dif = a.iter().cloned().collect::<HashSet<_>>();
    let dif_qty = dif.len();
    if dif_qty <= kk {
        out_line!(kk - dif.len());
        return;
    }
    let mut left = vec![None; n * n + 1];
    let mut right = vec![None; n * n + 1];
    let mut up = vec![None; n * n + 1];
    let mut down = vec![None; n * n + 1];
    for i in 0..n {
        for j in 0..n {
            let cur = a[(i, j)];
            left[cur].minim(j);
            right[cur].maxim(j);
            up[cur].minim(i);
            down[cur].maxim(i);
        }
    }
    let mut r = Vec::new();
    for i in 1..=n * n {
        if left[i].is_some() {
            r.push((
                up[i].unwrap(),
                left[i].unwrap(),
                down[i].unwrap(),
                right[i].unwrap(),
            ));
        }
    }
    let mut to_right = Arr2d::new(n, n, 0);
    let mut to_down = Arr2d::new(n, n, 0);
    for i in 0..n {
        for &(_, l, d, r) in &r {
            if l >= i {
                if r - i > d {
                    to_right[i][r] += 1;
                } else {
                    to_down[i][d] += 1;
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            let mut cur = dif_qty;
            for k in 0..(n - i).min(n - j) {
                cur -= to_right[j][j + k] + to_down[j][i + k];
                if cur == kk || cur + 1 == kk {
                    out_line!(1);
                    return;
                }
            }
        }
        for &(u, l, d, r) in &r {
            if u == i {
                for j in 0..=l {
                    if r - j > d - i {
                        to_right[j][r] -= 1;
                    } else {
                        to_down[j][d] -= 1;
                    }
                }
            } else if u > i {
                if i + r >= d {
                    let x = i + r - d;
                    if x <= l {
                        to_right[x][r] += 1;
                        to_down[x][d] -= 1;
                    }
                }
            }
        }
    }
    out_line!(2);
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
