//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;
// use std::collections::BinaryHeap;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_long_vec(n);

    #[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
    struct Segment {
        sum: i64,
        from: usize,
        to: usize,
    }

    impl Segment {
        fn join(&self, right: Segment) -> Self {
            assert_eq!(self.to, right.from);
            Self {
                sum: self.sum + right.sum,
                from: self.from,
                to: right.to,
            }
        }
    }

    #[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
    struct Side {
        best: Segment,
        left: Segment,
        right: Segment,
        all: Segment,
    }

    impl Side {
        fn single(pos: usize, value: i64) -> Self {
            Self {
                best: if value >= 0 {
                    Segment {
                        sum: value,
                        from: pos,
                        to: pos + 1,
                    }
                } else {
                    Segment {
                        sum: 0,
                        from: pos,
                        to: pos,
                    }
                },
                left: if value >= 0 {
                    Segment {
                        sum: value,
                        from: pos,
                        to: pos + 1,
                    }
                } else {
                    Segment {
                        sum: 0,
                        from: pos,
                        to: pos,
                    }
                },
                right: if value >= 0 {
                    Segment {
                        sum: value,
                        from: pos,
                        to: pos + 1,
                    }
                } else {
                    Segment {
                        sum: 0,
                        from: pos + 1,
                        to: pos + 1,
                    }
                },
                all: Segment {
                    sum: value,
                    from: pos,
                    to: pos + 1,
                },
            }
        }

        fn join(&self, right: Side) -> Self {
            Self {
                best: self.best.max(right.best).max(self.right.join(right.left)),
                left: self.left.max(self.all.join(right.left)),
                right: right.right.max(self.right.join(right.all)),
                all: self.all.join(right.all),
            }
        }
    }

    #[derive(Copy, Clone, Default, Eq, PartialEq)]
    struct Node {
        positive: Side,
        negative: Side,
        swap_down: bool,
    }

    impl Node {
        fn single(pos: usize, value: i64) -> Self {
            Self {
                positive: Side::single(pos, value),
                negative: Side::single(pos, -value),
                swap_down: false,
            }
        }

        fn join(&self, right: Node) -> Self {
            if *self == Node::default() {
                right
            } else if right == Node::default() {
                *self
            } else {
                Self {
                    positive: self.positive.join(right.positive),
                    negative: self.negative.join(right.negative),
                    swap_down: false,
                }
            }
        }

        fn swap(&mut self) {
            swap(&mut self.positive, &mut self.negative);
            self.swap_down ^= true;
        }
    }

    let mut tree = vec![Node::default(); 4 * n];
    let mut init = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
        if left + 1 == right {
            tree[root] = Node::single(left, a[left]);
            return;
        }
        let mid = (left + right) >> 1;
        f.call(2 * root + 1, left, mid);
        f.call(2 * root + 2, mid, right);
        let node = tree[2 * root + 1].join(tree[2 * root + 2]);
        tree[root] = node;
    });
    init.call(0, 0, n);

    let mut ans = tree[0].positive.all.sum;
    for _ in 0..k {
        ans += tree[0].negative.best.sum;
        let from = tree[0].negative.best.from;
        let to = tree[0].negative.best.to;
        let mut update = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
            if to <= left || right <= from {
                return;
            }
            if from <= left && right <= to {
                tree[root].swap();
                return;
            }
            let mid = (left + right) >> 1;
            if tree[root].swap_down {
                tree[2 * root + 1].swap();
                tree[2 * root + 2].swap();
                tree[root].swap_down = false;
            }
            f.call(2 * root + 1, left, mid);
            f.call(2 * root + 2, mid, right);
            tree[root] = tree[2 * root + 1].join(tree[2 * root + 2]);
            // unite
        });
        update.call(0, 0, n);
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
