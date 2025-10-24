//{"name":"I. Plants vs Zombies | Garlic","group":"KEP.uz","url":"https://kep.uz/competitions/contests/contest/452/problem/I","interactive":false,"timeLimit":4000,"tests":[{"input":"2 3\n010\n000\n3\nQ 1 1 1 3\nQ 1 1 2 3\nQ 2 1 2 3\n","output":"0.0000000000 1.0000000000\n1.0000000000 1.0000000000\n1.0000000000 0.0000000000\n"},{"input":"3 4\n0000\n0100\n0000\n3\nQ 2 1 1 3\nU 1 3 1\nQ 2 1 2 4\n","output":"​0.5000000000 1.0000000000\n0.5000000000 1.5000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::real::Real;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let n = input.read_size();
    let mut g = input.read_str_vec(m);

    fn build_matrix(g: &[Str], col: usize, mat: &mut Matrix<Real>) {
        mat.fill(Real::zero());
        mat[g.len()][g.len()] = Real::one();
        for i in 0..g.len() {
            if g[i][col] == b'0' || g.len() == 1 {
                mat[i][i] = Real::one();
            } else if i == 0 {
                mat[i][i + 1] = Real::one();
                mat[i][g.len()] = Real::one();
            } else if i + 1 == g.len() {
                mat[i][i - 1] = Real::one();
                mat[i][g.len()] = Real::one();
            } else {
                mat[i][i - 1] = Real(0.5);
                mat[i][i + 1] = Real(0.5);
                mat[i][g.len()] = Real::one();
            }
        }
    }
    #[derive(Default)]
    struct Node {
        mat: Matrix<Real>,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            if self.mat.d1() == 0 {
                self.mat = Matrix::zero(left_val.mat.d1(), left_val.mat.d1());
            }
            self.mat.do_mult(&left_val.mat, &right_val.mat);
        }
    }
    let mut st = SegmentTree::with_gen(n, |i| {
        let mut mat = Matrix::zero(m + 1, m + 1);
        build_matrix(&g, i, &mut mat);
        Node { mat }
    });

    let q = input.read_size();
    for _ in 0..q {
        let t = input.read_char();
        match t {
            b'U' => {
                let r = input.read_size() - 1;
                let c = input.read_size() - 1;
                let val = input.read_char();
                g[r][c] = val;
                st.for_each_mut(c..=c, |_, node| {
                    build_matrix(&g, c, &mut node.mat);
                });
            }
            b'Q' => {
                let rs = input.read_size() - 1;
                let cs = input.read_size() - 1;
                let rt = input.read_size() - 1;
                let ct = input.read_size() - 1;
                if cs > ct {
                    out.print_line((0, 0));
                    continue;
                }
                let mut ans = Matrix::ident(m + 1);
                let mut temp = Matrix::zero(m + 1, m + 1);
                st.for_each(cs..ct, |_, node| {
                    temp.do_mult(&ans, &node.mat);
                    swap(&mut ans, &mut temp);
                });
                out.print_line((ans[rs][rt], ans[rs][m]));
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
