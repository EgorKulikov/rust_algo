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
    let a = input.read_long_vec(n);

    let mut cur = 0;
    let mut before = vec![0];
    let mut cv = vec![0];
    let mut ans = 0;
    for i in 0..n {
        if a[i] < cur {
            ans += cur - a[i];
        } else {
            cur = a[i];
        }
        before.push(ans);
        cv.push(cur);
    }

    struct Node {
        sum: i64,
        qty: i64,
        self_val: i64,
        self_qty: i64,
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.self_val * self.self_qty
                + left.map_or(0, |l| l.sum)
                + right.map_or(0, |r| r.sum);
            self.qty = self.self_qty + left.map_or(0, |l| l.qty) + right.map_or(0, |r| r.qty);
        }
    }

    let mut ans = Some(ans);
    let mut right = Tree::<Node>::new();
    let mut after = 0;
    for i in (0..n).rev() {
        let mut cur = before[i] + after;
        right.binary_search(|node, left, _| {
            if node.self_val < cv[i] {
                cur += (node.self_qty + left.map_or(0, |l| l.qty)) * cv[i]
                    - node.self_val * node.self_qty
                    - left.map_or(0, |l| l.sum);
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        });
        ans.minim(cur);
        let mut nqty = 1;
        while let Some(node) = right.first() {
            if node.self_val <= a[i] {
                nqty += node.self_qty;
                after += (a[i] - node.self_val) * node.self_qty;
                right.get_at(0).detach();
            } else {
                break;
            }
        }
        right.add_front(Node {
            sum: a[i] * nqty,
            qty: nqty,
            self_val: a[i],
            self_qty: nqty,
        });
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
