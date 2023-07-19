//{"name":"A2: Perfectly Balanced - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/A2","interactive":false,"timeLimit":360000,"tests":[{"input":"1\n6\n2 1 5 1 5 1\n17\n2 2 6\n2 1 5\n2 1 3\n1 1 5\n1 2 5\n2 1 3\n2 1 5\n2 2 2\n1 3 1000000\n2 1 5\n2 1 3\n2 2 4\n2 4 6\n1 6 1000000\n2 4 6\n1 5 1000000\n2 5 6\n","output":"Case #1: 7\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"perfectly_balanced_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"perfectly_balanced_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"A2PerfectlyBalancedChapter2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::random::random;
use algo_lib::misc::recursive_function::{
    Callable3, Callable5, RecursiveFunction3, RecursiveFunction5,
};
use std::collections::{HashMap, HashSet};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        a: Vec<usize>,
        q: usize,
        queries: Vec<(usize, usize, usize)>,
        ans: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.a = input.read_vec(self.n);
            self.q = input.read();
            self.queries = input.read_vec(self.q);
        }

        fn solve(&mut self) {
            #[derive(Clone, Default, Copy, Hash, PartialEq, Eq)]
            struct Node {
                x1: u64,
                x2: u64,
            }

            impl Node {
                fn update_from(&mut self, other: Self) {
                    self.x1 = self.x1.wrapping_add(other.x1);
                    self.x2 = self.x2.wrapping_add(other.x2);
                }
            }

            let mut tree = vec![Node::default(); 4 * self.n];

            let mut vals = HashMap::new();
            let mut back = HashSet::new();

            let mut get = |x: usize| -> Node {
                if !vals.contains_key(&x) {
                    vals.insert(
                        x,
                        Node {
                            x1: random().gen(),
                            x2: random().gen(),
                        },
                    );
                    back.insert(vals[&x]);
                }
                vals[&x]
            };

            let mut init = RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                if left + 1 == right {
                    tree[root] = get(self.a[left]);
                    return;
                }
                let mid = (left + right) >> 1;
                // push down
                f.call(2 * root + 1, left, mid);
                f.call(2 * root + 2, mid, right);
                let node1 = tree[2 * root + 1];
                tree[root].update_from(node1);
                let node2 = tree[2 * root + 2];
                tree[root].update_from(node2);
            });
            init.call(0, 0, self.n);

            for &(t, l, r) in &self.queries {
                if t == 1 {
                    let mut get = |x: usize| -> Node {
                        if !vals.contains_key(&x) {
                            vals.insert(
                                x,
                                Node {
                                    x1: random().gen(),
                                    x2: random().gen(),
                                },
                            );
                            back.insert(vals[&x]);
                        }
                        vals[&x]
                    };

                    let node1 = get(r);
                    let node2 = get(self.a[l - 1]);
                    let node = Node {
                        x1: node1.x1.wrapping_sub(node2.x1),
                        x2: node1.x2.wrapping_sub(node2.x2),
                    };
                    let at = l - 1;
                    let mut point_op =
                        RecursiveFunction3::new(|f, root: usize, left: usize, right: usize| {
                            tree[root].update_from(node);

                            if left + 1 == right {
                                return;
                            }

                            let mid = (left + right) >> 1;
                            if at < mid {
                                f.call(2 * root + 1, left, mid);
                            } else {
                                f.call(2 * root + 2, mid, right);
                            }
                        });
                    point_op.call(0, 0, self.n);
                    self.a[l - 1] = r;
                } else {
                    if l % 2 != r % 2 {
                        continue;
                    }
                    let m = (l + r) / 2;
                    let mut segment_op = RecursiveFunction5::new(
                        |f,
                         root: usize,
                         left: usize,
                         right: usize,
                         from: usize,
                         to: usize|
                         -> Node {
                            if to <= left || right <= from {
                                return Node::default();
                            }
                            if from <= left && right <= to {
                                return tree[root];
                            }
                            let mid = (left + right) >> 1;
                            let mut res = f.call(2 * root + 1, left, mid, from, to);
                            res.update_from(f.call(2 * root + 2, mid, right, from, to));
                            res
                        },
                    );

                    let res1 = segment_op.call(0, 0, self.n, l - 1, m);
                    let res2 = segment_op.call(0, 0, self.n, m, r);
                    let res = Node {
                        x1: res1.x1.wrapping_sub(res2.x1),
                        x2: res1.x2.wrapping_sub(res2.x2),
                    };
                    if back.contains(&res) {
                        self.ans += 1;
                        continue;
                    }
                    let res1 = segment_op.call(0, 0, self.n, l - 1, m - 1);
                    let res2 = segment_op.call(0, 0, self.n, m - 1, r);
                    let res = Node {
                        x1: res2.x1.wrapping_sub(res1.x1),
                        x2: res2.x2.wrapping_sub(res1.x2),
                    };
                    if back.contains(&res) {
                        self.ans += 1;
                        continue;
                    }
                }
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case), self.ans);
        }
    }

    run_parallel::<Job>(input);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
