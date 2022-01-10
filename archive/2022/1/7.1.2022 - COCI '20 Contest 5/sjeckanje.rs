//{"name":"#5 - Sjeckanje","group":"DMOJ - COCI '20 Contest 5","url":"https://dmoj.ca/problem/coci20c5p5","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n1 2 3 4\n1 2 1\n1 1 2\n2 3 1\n","output":"2\n2\n0\n"},{"input":"4 3\n2 0 2 1\n4 4 1\n2 2 3\n1 3 2\n","output":"2\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Sjeckanje"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let q = input.read_usize();
    let a = input.read_long_vec(n);

    const INFTY: i64 = i64::MIN / 3;

    #[derive(Default, Copy, Clone)]
    struct Node {
        delta: i64,
        val: [[i64; 3]; 3],
        length: usize,
    }

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            let mut res = Self {
                length: right - left,
                ..Default::default()
            };
            if res.length == 1 {
                res.val[1][1] = INFTY;
                res.val[2][2] = INFTY;
            }
            res
        }

        fn join(&mut self, left: &Self, right: &Self) {
            for v in &mut self.val {
                for j in v {
                    *j = INFTY;
                }
            }
            for i in 0..3 {
                for j in 0..3 {
                    for k in 0..3 {
                        let l = (3 - j) % 3;
                        self.val[i][k].maxim(left.val[i][j] + right.val[l][k]);
                    }
                }
            }
        }

        fn accumulate(&mut self, value: &Self) {
            self.delta += value.delta;
            for i in 0..3 {
                for j in 0..3 {
                    if self.length == 1 && i == j && i > 0 {
                        if i == j && i > 0 {
                            self.val[i][i] = INFTY;
                        }
                    } else {
                        if i == 1 {
                            self.val[i][j] -= value.delta;
                        } else if i == 2 {
                            self.val[i][j] += value.delta;
                        }
                        if j == 1 {
                            self.val[i][j] -= value.delta;
                        } else if j == 2 {
                            self.val[i][j] += value.delta;
                        }
                    }
                }
            }
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }
    let mut st = SegmentTree::from_generator(n, |i| {
        let mut res = Node::new(i, i + 1);
        res.accumulate(&Node {
            delta: a[i],
            ..Default::default()
        });
        res
    });
    for _ in 0..q {
        let l = input.read_usize() - 1;
        let r = input.read_usize();
        let x = input.read_long();
        // for i in &mut a[l..r] {
        //     *i += x;
        // }
        st.update(
            l,
            r,
            &Node {
                delta: x,
                ..Default::default()
            },
        );
        /*let mut no_min = None;
        let mut no_max = None;
        let mut ans = 0;
        for &i in &a {
            let mut n_ans = ans;
            if let Some(a) = no_min {
                n_ans.maxim(a - i);
            }
            no_min.maxim(ans + i);
            if let Some(a) = no_max {
                n_ans.maxim(a + i);
            }
            no_max.maxim(ans - i);
            ans = n_ans;
        }
        out_line!(ans);*/
        out_line!(st.query(0, n).val[0][0]);
    }
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
