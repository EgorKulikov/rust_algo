use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_long_vec(n);

    struct Node {
        val: i64,
        sum: i64,
        ans: i64,
        len: i64,
        key: (i64, usize),
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
            self.len = 1 + left.map_or(0, |l| l.len) + right.map_or(0, |r| r.len);
            let right_len = right.map_or(0, |l| l.len);
            self.ans = right.map_or(0, |l| l.ans) + self.val * (right_len + 1) + left.map_or(0, |r| r.ans + r.sum * (right_len + 1));
        }
    }
    impl OrdPayload for Node {
        type Key = (i64, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }
    impl Node {
        fn new(delta: i64, id: usize) -> Self {
            Self {
                val: delta,
                sum: delta,
                ans: delta,
                len: 1,
                key: (delta, id),
            }
        }
    }
    let mut tree = Tree::new();
    for i in 1..n {
        tree.insert(Node::new(a[i] - a[i - 1], i));
    }

    for _ in 0..q {
        let at = input.read_size() - 1;
        let x = input.read_long();
        for i in at.max(1)..(at + 2).min(n) {
            tree.remove(&(a[i] - a[i - 1], i));
            let w = a[at];
            a[at] = x;
            tree.insert(Node::new(a[i] - a[i - 1], i));
            a[at] = w;
        }
        a[at] = x;
        out.print_line(tree.payload().map_or(0, |p| p.ans) + a[0] * n as i64);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
