//{"name":"Mean Difference (Hard)","group":"CodeChef - START213A","url":"https://www.codechef.com/START213A/problems/MEANDIFF2","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n4\n8 5 9 1\n3\n8 8 8\n5\n3 9 48 8 61\n","output":"0 2 2 5\n0 0 0\n0 3 28 31 41\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
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
        size: usize,
        key: (usize, usize),
    }

    impl Node {
        fn new(val: usize, id: usize) -> Self {
            Self {
                val,
                sum: val,
                key: (val, id),
                size: 1,
            }
        }
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
            self.size = 1 + left.map_or(0, |l| l.size) + right.map_or(0, |r| r.size);
        }
    }

    impl OrdPayload for Node {
        type Key = (usize, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }

    let mut ans = Vec::with_capacity(n);
    let mut tree = Tree::new();
    let mut min = usize::MAX;
    let mut max = 0;
    for i in 0..n {
        tree.insert(Node::new(a[i], i));
        min.minim(a[i]);
        max.maxim(a[i]);
        let mut cur = 0;
        let mut sum = 0;
        let mut size = 0;
        tree.range_index(..i).binary_search(|node, left, _| {
            let (left_sum, left_size) = match left {
                Some(l) => (l.sum, l.size),
                None => (0, 0),
            };
            let left_val = (left_sum + max + sum) / (left_size + 1 + size);
            cur.maxim(max - left_val);
            let right_val = (left_sum + node.val + max + sum) / (left_size + 2 + size);
            cur.maxim(max - right_val);
            if ((left_sum + max + sum) as i128) * ((left_size + 2 + size) as i128)
                < ((left_sum + node.val + max + sum) as i128) * ((left_size + 1 + size) as i128)
            {
                Some(Direction::Left)
            } else {
                sum += left_sum + node.val;
                size += left_size + 1;
                Some(Direction::Right)
            }
        });
        sum = 0;
        size = 0;
        tree.range_index(1..).binary_search(|node, _, right| {
            let (right_sum, right_size) = match right {
                Some(r) => (r.sum, r.size),
                None => (0, 0),
            };
            let right_val = (right_sum + min + sum) / (right_size + 1 + size);
            cur.maxim(right_val - min);
            let left_val = (right_sum + node.val + min + sum) / (right_size + 2 + size);
            cur.maxim(left_val - min);
            if ((right_sum + node.val + min + sum) as i128) * ((right_size + 1 + size) as i128)
                > ((right_sum + min + sum) as i128) * ((right_size + 2 + size) as i128)
            {
                sum += right_sum + node.val;
                size += right_size + 1;
                Some(Direction::Left)
            } else {
                Some(Direction::Right)
            }
        });
        ans.push(cur);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
