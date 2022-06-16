//{"name":"F. Fast Food","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/F/","interactive":false,"timeLimit":3000,"tests":[{"input":"5 2\n2 2\n1 3 6 4 1\n5 2 3 1 1\n","output":"9\n1 1 2 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFastFood"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_usize_vec(m);
    let c = input.read_table::<i32>(m, n);

    let mut cur = Arr2d::new(
        n + 1,
        n + 1,
        ((i32::MAX, 0, 0, false), (i32::MAX, 0, 0, false)),
    );
    cur[(0, 0)] = ((0, 0, 0, false), (0, 0, 0, false));
    for i in 0..n {
        let mut ch = Vec::with_capacity(m);
        for j in 0..m {
            ch.push((c[(j, i)], j));
        }
        ch.sort_unstable();
        ch.truncate(3);
        for j in 0..=i {
            let (first, second) = cur[(i, j)];
            if first.0 == i32::MAX {
                continue;
            }
            let mut try_to = |days: usize, cost: (i32, usize, usize, bool)| {
                let (first, second) = &mut cur[(i + 1, days)];
                if first.1 == cost.1 {
                    first.minim(cost);
                    return;
                }
                let fc = *first;
                if first.minim(cost) {
                    *second = fc;
                } else {
                    second.minim(cost);
                }
            };
            for &(c, id) in &ch {
                if id == first.1 {
                    if j != a[id] {
                        try_to(j + 1, (first.0 + c, id, j, false));
                    }
                    if second.0 != i32::MAX {
                        try_to(1, (second.0 + c, id, j, true));
                    }
                } else {
                    try_to(1, (first.0 + c, id, j, false));
                }
            }
        }
    }
    let mut ans = None;
    for &(cost, _) in cur.row(n) {
        ans.minim(cost);
    }
    out_line!(ans.unwrap().0);
    let mut row = n;
    let mut who = ans.unwrap().1;
    let mut col = ans.unwrap().2;
    let mut way = ans.unwrap().3;
    let mut res = Vec::with_capacity(n);
    while row > 0 {
        res.push(who + 1);
        let n_who = if way {
            cur[(row - 1, col)].1 .1
        } else {
            cur[(row - 1, col)].0 .1
        };
        let n_col = if way {
            cur[(row - 1, col)].1 .2
        } else {
            cur[(row - 1, col)].0 .2
        };
        let n_way = if way {
            cur[(row - 1, col)].1 .3
        } else {
            cur[(row - 1, col)].0 .3
        };
        row -= 1;
        who = n_who;
        col = n_col;
        way = n_way;
    }
    res.reverse();
    out_line!(res);
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
