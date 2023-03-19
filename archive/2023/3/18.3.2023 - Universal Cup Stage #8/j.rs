//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_vec(n);

    let s = a.as_slice().partial_sums();
    type Point = (i128, i128);
    type Node = Vec<Point>;

    fn over(p1: Point, p2: Point, p3: Point) -> bool {
        p1.1 * (p3.0 - p2.0) + p3.1 * (p2.0 - p1.0) <= p2.1 * (p3.0 - p1.0)
    }

    fn enlarge(node: &mut Node, point: Point) {
        loop {
            if let Some(&last) = node.last() {
                if last.1 >= point.1 {
                    node.pop();
                    continue;
                }
            }
            if node.len() >= 2 {
                let last = *node.last().unwrap();
                let pre_last = node[node.len() - 2];
                if over(pre_last, last, point) {
                    node.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        node.push(point);
    }
    let mut tree = vec![Node::new(); 4 * (n + 1)];
    let mut init = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
        if left + 1 == right {
            tree[root].push((left.into_i128(), s[left]));
            return;
        }
        let mid = (left + right) >> 1;
        f.call(2 * root + 1, left, mid);
        f.call(2 * root + 2, mid, right);
        tree[root] = tree[2 * root + 1].clone();
        let (head, tail) = tree.split_at_mut(2 * root + 2);
        for &point in &tail[0] {
            enlarge(&mut head[root], point);
        }
    });
    init.call(0, 0, n + 1);

    for _ in 0..m {
        let start = input.read_size() - 1;
        let end = start + input.read_size();

        let sp = (start.into_i128(), s[start]);
        let from = start + 1;
        let to = end + 1;
        let mut ans = None;
        let mut segment_op =
            RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                if to <= left || right <= from {
                    return;
                }
                if from <= left && right <= to {
                    let t = &tree[root];
                    if t[0].1 < sp.1 {
                        ans.minim(-1);
                        return;
                    }
                    let mut left = 0;
                    let mut right = t.len() - 1;
                    while left < right {
                        let mid = (left + right) >> 1;
                        if over(sp, t[mid], t[mid + 1]) {
                            left = mid + 1;
                        } else {
                            right = mid;
                        }
                    }
                    ans.minim((t[left].1 - sp.1) / (t[left].0 - sp.0));
                    return;
                }
                let mid = (left + right) >> 1;
                f.call(2 * root + 1, left, mid);
                f.call(2 * root + 2, mid, right);
            });
        segment_op.call(0, 0, n + 1);
        let ans = ans.unwrap();
        if ans == -1 {
            out_line!("stay with parents");
        } else {
            out_line!(ans);
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
