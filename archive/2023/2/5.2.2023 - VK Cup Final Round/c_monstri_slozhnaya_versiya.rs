//{"name":"C. Монстры (сложная версия)","group":"Codeforces - VK Cup 2022 - Финальный раунд (Engine)","url":"https://codeforces.com/contest/1784/problem/C","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3\n3 1 2\n6\n4 1 5 4 1 1\n","output":"2 1 0\n3 2 4 4 4 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMonstriSlozhnayaVersiya"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::permutation::Permutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{
    Callable, Callable3, RecursiveFunction, RecursiveFunction3,
};
use algo_lib::out_line;

fn solve_impl(n: usize, a: Vec<usize>) -> Vec<usize> {
    #[derive(Copy, Clone)]
    struct Node {
        enabled: usize,
        min_sub: usize,
        delta_min_sub: usize,
    }

    impl Node {
        fn new() -> Self {
            Self {
                enabled: 0,
                min_sub: usize::MAX / 2,
                delta_min_sub: 0,
            }
        }

        fn push_down(&mut self, left: &mut Self, right: &mut Self) {
            left.min_sub -= self.delta_min_sub;
            left.delta_min_sub += self.delta_min_sub;
            right.min_sub -= self.delta_min_sub;
            right.delta_min_sub += self.delta_min_sub;
            self.delta_min_sub = 0;
        }
    }
    let mut ans = Vec::with_capacity(n);
    let mut cur = 0;
    let mut tree = vec![Node::new(); 4 * n];
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| a[i]);
    let pos = Permutation::new(order.clone()).inv().to_vec();
    for i in 0..n {
        let to = pos[i];
        let mut segment_op = RecursiveFunction3::new(
            |f, root: usize, left: usize, right: usize| -> Option<usize> {
                if to <= left {
                    return None;
                }
                if right <= to {
                    if tree[root].enabled == 0 {
                        return None;
                    }
                    if right - left == 1 {
                        return Some(a[order[left]] - tree[root].min_sub);
                    }
                    let (head, tail) = tree.split_at_mut(2 * root + 1);
                    let (l_child, r_child) = tail.split_at_mut(1);
                    head[root].push_down(&mut l_child[0], &mut r_child[0]);
                    let mid = (left + right) >> 1;
                    if tree[2 * root + 2].enabled > 0 {
                        return f.call(2 * root + 2, mid, right);
                    } else {
                        return f.call(2 * root + 1, left, mid);
                    }
                }
                let (head, tail) = tree.split_at_mut(2 * root + 1);
                let (l_child, r_child) = tail.split_at_mut(1);
                head[root].push_down(&mut l_child[0], &mut r_child[0]);
                let mid = (left + right) >> 1;
                // push down
                let res = f.call(2 * root + 2, mid, right);
                if res.is_some() {
                    return res;
                }
                f.call(2 * root + 1, left, mid)
            },
        );
        let val_left = segment_op.call(0, 0, n).unwrap_or(0);
        let c_value = if a[i] == val_left {
            val_left
        } else {
            let from = pos[i] + 1;
            let mut segment_op =
                RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                    if right <= from {
                        return true;
                    }
                    if from <= left {
                        if tree[root].enabled == 0 {
                            return true;
                        }
                        if tree[root].min_sub > 0 {
                            tree[root].min_sub -= 1;
                            tree[root].delta_min_sub += 1;
                            cur -= tree[root].enabled;
                            return true;
                        }
                        if right - left == 1 {
                            return false;
                        }
                        let mid = (left + right) >> 1;
                        let (head, tail) = tree.split_at_mut(2 * root + 1);
                        let (l_child, r_child) = tail.split_at_mut(1);
                        head[root].push_down(&mut l_child[0], &mut r_child[0]);
                        if f.call(2 * root + 1, left, mid) {
                            f.call(2 * root + 2, mid, right);
                        }
                        tree[root].min_sub =
                            tree[2 * root + 1].min_sub.min(tree[2 * root + 2].min_sub);
                        return false;
                    }
                    let mid = (left + right) >> 1;
                    // push down
                    let (head, tail) = tree.split_at_mut(2 * root + 1);
                    let (l_child, r_child) = tail.split_at_mut(1);
                    head[root].push_down(&mut l_child[0], &mut r_child[0]);
                    let res = if f.call(2 * root + 1, left, mid) {
                        f.call(2 * root + 2, mid, right)
                    } else {
                        false
                    };
                    tree[root].min_sub = tree[2 * root + 1].min_sub.min(tree[2 * root + 2].min_sub);
                    res
                });
            segment_op.call(0, 0, n);
            val_left + 1
        };
        let cur_delta = a[i] - c_value;
        cur += cur_delta;
        ans.push(cur);
        let at = pos[i];
        let mut point_op = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            tree[root].enabled += 1;
            tree[root].min_sub.minim(cur_delta);
            if left + 1 == right {
                return;
            }
            let (head, tail) = tree.split_at_mut(2 * root + 1);
            let (l_child, r_child) = tail.split_at_mut(1);
            head[root].push_down(&mut l_child[0], &mut r_child[0]);
            let mid = (left + right) >> 1;
            if at < mid {
                f.call(2 * root + 1, left, mid);
            } else {
                f.call(2 * root + 2, mid, right);
            }
            tree[root].min_sub = tree[2 * root + 1].min_sub.min(tree[2 * root + 2].min_sub);
        });
        point_op.call(0, 0, n);
    }
    ans
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    out_line!(solve_impl(n, a));
}

#[test]
fn test() {
    for n in 5..=6 {
        let mut a = vec![0; n];
        let mut rec = RecursiveFunction::new(|f, i: usize| {
            if i == n {
                // println!("a = {:?}", a);
                let actual = solve_impl(n, a.clone());
                let mut expected = Vec::with_capacity(n);
                fn solve_slow(a: &[usize]) -> usize {
                    let mut a = a.to_vec();
                    a.sort();
                    let mut last = 0;
                    let mut ans = 0;
                    for i in a {
                        last += 1;
                        if last <= i {
                            ans += i - last;
                        } else {
                            last = i;
                        }
                    }
                    ans
                }
                for i in 0..n {
                    expected.push(solve_slow(&a[..=i]));
                }
                assert_eq!(actual, expected);
                return;
            }
            for j in 1..=n {
                a[i] = j;
                f.call(i + 1);
            }
        });
        rec.call(0);
    }
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
