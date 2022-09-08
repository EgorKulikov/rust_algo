//{"name":"F. Опоздать на работу","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"5 10\n4 2\n7 3\n3 6\n5 2\n8 0\n1 2 3 4\n","output":"11\n"},{"input":"6 9\n5 3\n5 5\n7 0\n5 8\n7 7\n6 6\n0 0 0 0 0\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FOpozdatNaRabotu"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let t = input.read_long();
    let lights = input.read_long_pair_vec(n);
    let d = input.read_long_vec(n - 1);

    let mut poi = Vec::with_capacity(2 * n);
    let mut sum = 0;
    let mut green = Vec::with_capacity(n);
    let mut red = Vec::with_capacity(n);
    for i in 0..n {
        let (g, c) = lights[i];
        let ct = (c + sum) % t;
        let c_green = (t - ct) % t;
        let c_red = (c_green + g) % t;
        poi.push(c_green);
        poi.push(c_red);
        green.push(c_green);
        red.push(c_red);
        if i < n - 1 {
            sum += d[i];
        }
    }
    poi.sort();
    poi.dedup();
    let m = poi.len();
    let mut tree = vec![0; 4 * m];

    const INFTY: i64 = i64::MAX / 2;
    fn fill_infty(tree: &mut [i64], l: usize, r: usize) {
        let from = l;
        let to = r;
        let m = tree.len() / 4;
        let mut segment_op =
            RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                if to <= left || right <= from {
                    return;
                }
                if tree[root] == INFTY {
                    return;
                }
                if from <= left && right <= to {
                    tree[root] = INFTY;
                    return;
                }
                let mid = (left + right) >> 1;
                f.call(2 * root + 1, left, mid);
                f.call(2 * root + 2, mid, right);
                let val = tree[2 * root + 1].min(tree[2 * root + 2]);
                tree[root] = val;
            });
        segment_op.call(0, 0, m);
    }
    fn set_point(tree: &mut [i64], at: usize, val: i64) {
        let m = tree.len() / 4;
        let mut point_op = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            if left + 1 == right {
                tree[root].minim(val);
                return;
            }
            if tree[root] == INFTY {
                tree[2 * root + 1] = INFTY;
                tree[2 * root + 2] = INFTY;
            }
            tree[root].minim(val);
            let mid = (left + right) >> 1;
            if at < mid {
                f.call(2 * root + 1, left, mid);
            } else {
                f.call(2 * root + 2, mid, right);
            }
        });
        point_op.call(0, 0, m);
    }
    fn get(tree: &[i64], l: usize, r: usize) -> i64 {
        let from = l;
        let to = r;
        let mut segment_op =
            RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                if to <= left || right <= from {
                    return INFTY;
                }
                if tree[root] == INFTY {
                    return INFTY;
                }
                if from <= left && right <= to {
                    return tree[root];
                }
                let mid = (left + right) >> 1;
                f.call(2 * root + 1, left, mid)
                    .min(f.call(2 * root + 2, mid, right))
            });
        segment_op.call(0, 0, tree.len() / 4)
    }

    for i in 0..m {
        set_point(
            &mut tree,
            i,
            if i == m - 1 { 1 - t } else { 1 - poi[i + 1] },
        );
    }

    for i in 0..n {
        let l = poi.binary_search(&red[i]).unwrap();
        let r = poi.binary_search(&green[i]).unwrap();
        if l < r {
            let cur = get(&tree, l, r);
            fill_infty(&mut tree, l, r);
            set_point(&mut tree, r, cur);
        } else {
            let cur = (get(&tree, l, m) + t).min(get(&tree, 0, r));
            fill_infty(&mut tree, l, m);
            fill_infty(&mut tree, 0, r);
            set_point(&mut tree, r, cur);
        }
    }
    let mut f = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
        if left + 1 == right {
            return (tree[root] + poi[left]).max(0);
        }
        if tree[root] == INFTY {
            return INFTY;
        }
        let mid = (left + right) >> 1;
        f.call(2 * root + 1, left, mid)
            .min(f.call(2 * root + 2, mid, right))
    });
    let ans = f.call(0, 0, m) + sum;
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
