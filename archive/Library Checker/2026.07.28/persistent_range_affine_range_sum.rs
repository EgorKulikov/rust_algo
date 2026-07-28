use algo_lib::collections::payload::Payload;
use algo_lib::collections::treap::persistent_treap::PersistentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_unsigned_vec(n);

    type Mod = ModIntF;
    #[derive(Clone)]
    struct Node {
        self_val: Mod,
        val: Mod,
        qty: Mod,
        k: Mod,
        b: Mod,
    }
    impl Node {
        fn new(a: u32) -> Self {
            Self {
                self_val: a.into(),
                val: a.into(),
                qty: Mod::one(),
                k: Mod::one(),
                b: Mod::zero(),
            }
        }
    }
    impl Payload for Node {
        const NEED_ACCUMULATE: bool = true;
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.val = self.self_val + left.map_or(Mod::zero(), |l| l.val) + right.map_or(Mod::zero(), |r| r.val);
            self.qty = Mod::one() + left.map_or(Mod::zero(), |l| l.qty) + right.map_or(Mod::zero(), |r| r.qty);
        }

        fn accumulate(&mut self, delta: &Self) {
            self.self_val = delta.k * self.self_val + delta.b;
            self.val = delta.k * self.val + delta.b * self.qty;
            self.k *= delta.k;
            self.b = delta.k * self.b + delta.b;
        }

        fn accumulate_self(&mut self, delta: &Self) {
            self.self_val = delta.k * self.self_val + delta.b;
        }

        fn reset_delta(&mut self) {
            self.k = Mod::one();
            self.b = Mod::zero();
        }

        fn need_push_down(&self) -> bool {
            self.k != Mod::one() || self.b != Mod::zero()
        }
    }
    let mut trees = vec![PersistentTree::with_gen(n, |i| Node::new(a[i]))];

    for _ in 0..q {
        let t = input.read_int();
        match t {
            0 => {
                let k = (input.read_int() + 1) as usize;
                let l = input.read_size();
                let r = input.read_size();
                let b: Mod = input.read();
                let c: Mod = input.read();
                let delta = Node {
                    self_val: Mod::zero(),
                    val: Mod::zero(),
                    qty: Mod::zero(),
                    k: b,
                    b: c,
                };
                trees.push(trees[k].push_range(l..r, &delta));
            }
            1 => {
                let k = (input.read_int() + 1) as usize;
                let s = (input.read_int() + 1) as usize;
                let l = input.read_size();
                let r = input.read_size();
                let (left, _, right) = trees[k].split_range_index(l..r);
                let mid = trees[s].range_index(l..r);
                trees.push(PersistentTree::merge_three(left, mid, right));
            }
            2 => {
                let k = (input.read_int() + 1) as usize;
                let l = input.read_size();
                let r = input.read_size();
                let cur = trees[k];
                trees.push(cur);
                out.print_line(cur.range_payload(l..r).unwrap().val);
            }
            _ => unreachable!(),
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
