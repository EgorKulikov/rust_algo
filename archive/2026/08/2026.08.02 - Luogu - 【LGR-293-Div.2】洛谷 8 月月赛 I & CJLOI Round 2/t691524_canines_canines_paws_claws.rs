use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::value;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    value!(Modulo: u32 = 19990721);
    type Mod = ModInt<Modulo>;

    #[derive(Default, Clone)]
    struct Matrix {
        v: Vec<Mod>,
    }

    impl Matrix {
        fn ident() -> Self {
            Self {
                v: vec![Mod::one(), Mod::zero(), Mod::zero(), Mod::one()],
            }
        }

        fn mult(a: &Matrix, b: &Matrix) -> Self {
            Self {
                v: vec![
                    a.v[0] * b.v[0] + a.v[1] * b.v[2],
                    a.v[0] * b.v[1] + a.v[1] * b.v[3],
                    a.v[2] * b.v[0] + a.v[3] * b.v[2],
                    a.v[2] * b.v[1] + a.v[3] * b.v[3],
                ],
            }
        }
    }

    #[derive(Default)]
    struct Node {
        dir: bool,
        start: usize,
        len: usize,
        mat: Matrix,
        inv_mat: Matrix,
        self_mat: Matrix,
        self_inv_mat: Matrix,
        delta: bool,
    }

    impl Node {
        fn new(start: usize, len: usize, dir: bool) -> Self {
            let mut base = Matrix::ident();
            if dir {
                base.v[1] = Mod::from(len);
            } else {
                base.v[2] = Mod::from(len);
            }
            let mut base_inv = Matrix::ident();
            if dir {
                base_inv.v[2] = Mod::from(len);
            } else {
                base_inv.v[1] = Mod::from(len);
            }
            Self {
                dir,
                start,
                len,
                mat: base.clone(),
                inv_mat: base_inv.clone(),
                self_mat: base,
                self_inv_mat: base_inv,
                delta: false,
            }
        }
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        const NEED_ACCUMULATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            assert!(!self.delta);
            self.mat = Matrix::ident();
            self.inv_mat = Matrix::ident();
            if let Some(left) = left {
                self.mat = Matrix::mult(&self.mat, &left.mat);
                self.inv_mat = Matrix::mult(&self.inv_mat, &left.inv_mat);
            }
            self.mat = Matrix::mult(&self.mat, &self.self_mat);
            self.inv_mat = Matrix::mult(&self.inv_mat, &self.self_inv_mat);
            if let Some(right) = right {
                self.mat = Matrix::mult(&self.mat, &right.mat);
                self.inv_mat = Matrix::mult(&self.inv_mat, &right.inv_mat);
            }
        }

        fn accumulate(&mut self, delta: &Self) {
            if delta.delta {
                self.delta ^= true;
                self.dir ^= true;
                swap(&mut self.mat, &mut self.inv_mat);
                swap(&mut self.self_mat, &mut self.self_inv_mat);
            }
        }

        fn reset_delta(&mut self) {
            self.delta = false;
        }
    }

    impl OrdPayload for Node {
        type Key = usize;

        fn key(&self) -> &Self::Key {
            &self.start
        }
    }

    let mut tree = Tree::single(Node::new(0, n - 1, true));
    let mut last_ans = 0;

    for _ in 0..m {
        let o = input.read_size();
        let l = (input.read_size() + last_ans) % n;
        let r = (input.read_size() + last_ans) % n + 1 - o;

        // debug!(o, l, r);
        assert!(l <= n - 1);
        assert!(r <= n - 1);

        if let Some(x) = tree.range(..&l).last() {
            if x.start + x.len != l {
                assert!(x.start + x.len > l);
                let mut first = Some(Node::new(x.start, l - x.start, x.dir));
                let mut second = Some(Node::new(l, x.start + x.len - l, x.dir));
                let pos = x.start;
                let x = tree.range(&pos..=&pos);
                *x = Tree::with_gen(2, |i| if i == 0 { first.take().unwrap() } else { second.take().unwrap() });
            }
        } else {
            assert_eq!(l, 0);
        }

        if let Some(x) = tree.range(..&r).last() {
            if x.start + x.len != r {
                assert!(x.start + x.len > r);
                let mut first = Some(Node::new(x.start, r - x.start, x.dir));
                let mut second = Some(Node::new(r, x.start + x.len - r, x.dir));
                let pos = x.start;
                let x = tree.range(&pos..=&pos);
                *x = Tree::with_gen(2, |i| if i == 0 { first.take().unwrap() } else { second.take().unwrap() });
            }
        } else {
            assert_eq!(r, 0);
        }

        if o == 0 {
            tree.range(&l..&r).push(&Node {
                delta: true,
                ..Node::default()
            });
        } else {
            let k = input.read_int();
            if k == 0 {
                out.print_line(1);
                last_ans = 1;
            } else if k == 1 {
                // eprintln!("==========");
                // for node in tree.iter() {
                //     eprintln!("{} {} {} {:?} {:?}", node.start, node.len, node.dir, node.self_mat.v, node.mat.v);
                // }
                let mut ans = Mod::new(2);
                if let Some(p) = tree.range(&l..&r).payload() {
                    // eprintln!("{} {} {} {:?} {:?}", p.start, p.len, p.dir, p.self_mat.v, p.mat.v);
                    ans = Mod::zero();
                    for x in p.mat.v.iter() {
                        ans += *x;
                    }
                }
                out.print_line(ans);
                // eprintln!("{}", ans);
                last_ans = ans.val() as usize;
            } else {
                out.print_line(2);
                last_ans = 2;
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
