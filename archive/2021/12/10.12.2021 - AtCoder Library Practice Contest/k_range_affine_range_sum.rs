//{"name":"K - Range Affine Range Sum","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_k","interactive":false,"timeLimit":5000,"tests":[{"input":"5 7\n1 2 3 4 5\n1 0 5\n0 2 4 100 101\n1 0 3\n0 1 3 102 103\n1 2 5\n0 2 5 104 105\n1 0 5\n","output":"15\n404\n41511\n4317767\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KRangeAffineRangeSum"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModIntF};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    type Mod = ModIntF;

    #[derive(Copy, Clone)]
    struct Node {
        left: i32,
        right: i32,
        sum: Mod,
        b: Mod,
        c: Mod,
    }

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Node {
                left: left as i32,
                right: right as i32,
                sum: Mod::zero(),
                b: Mod::one(),
                c: Mod::zero(),
            }
        }

        fn join(&mut self, left: &Self, right: &Self) {
            self.sum = left.sum + right.sum;
        }

        fn accumulate(&mut self, value: &Self) {
            self.sum = self.sum * value.b + Mod::new(self.right - self.left) * value.c;
            self.b *= value.b;
            self.c = self.c * value.b + value.c;
        }

        fn reset_delta(&mut self) {
            self.b = Mod::one();
            self.c = Mod::zero();
        }
    }

    let n = input.read();
    let q = input.read();
    let a = input.read_vec(n);

    let mut st = SegmentTree::from_generator(n, |at| Node {
        left: at as i32,
        right: at as i32 + 1,
        sum: Mod::new(a[at]),
        b: Mod::one(),
        c: Mod::new(a[at]),
    });

    for _ in 0usize..q {
        let t: u8 = input.read();
        let l = input.read();
        let r = input.read();

        if t == 0 {
            let b = input.read();
            let c = input.read();
            let node = Node {
                left: 0,
                right: 0,
                sum: Mod::zero(),
                b,
                c,
            };
            st.update(l, r, &node);
        } else {
            out_line!(st.query(l, r).sum);
        }
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
