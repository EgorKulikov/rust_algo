use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::Payload;
use algo_lib::collections::treap::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    struct Node {
        val: usize,
        sum: usize,
        len: usize,
        delta: Option<usize>,
    }
    impl Payload for Node {
        const NEED_ACCUMULATE: bool = true;
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
            self.len = 1 + left.map_or(0, |l| l.len) + right.map_or(0, |r| r.len);
        }

        fn accumulate(&mut self, delta: &Self) {
            if let Some(val) = delta.delta {
                self.val = val;
                self.sum = val * self.len;
                self.delta = Some(val);
            }
        }
        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }
    let mut ans = 0;
    let mut tree = Tree::new();
    let mut val = vec![None; n + 1];
    for i in 0..n {
        tree.add_back(Node {
            val: 0,
            sum: 0,
            len: 1,
            delta: None,
        });
        val[0] = Some(i);
        if let Some(v) = val[a[i] - 1] {
            val[a[i]].maxim(v);
            let head = tree.range_index(0..=v);
            let mut need_update = 0;
            head.binary_search_with_size(|node, _, _, _, right_size| {
                if node.val < a[i] {
                    need_update += right_size + 1;
                    Some(Direction::Left)
                } else {
                    Some(Direction::Right)
                }
            });
            head.range_index(v + 1 - need_update..).push(&Node {
                val: 0,
                sum: 0,
                len: 0,
                delta: Some(a[i]),
            });
        }
        ans += tree.payload().map_or(0, |p| p.sum);
    }
    out.print_line(ans);
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
