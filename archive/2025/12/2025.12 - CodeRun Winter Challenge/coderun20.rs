//{"name":"coderun20","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::ops::DerefMut;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_size_vec(n);

    enum Node {
        Leaf {
            qty: usize,
        },
        None,
        Node {
            left: Box<Node>,
            right: Box<Node>,
            ans: Option<(usize, bool)>,
        },
    }

    impl Node {
        fn get_ans(&mut self) -> (usize, bool) {
            match self {
                Node::Leaf { .. } => (1, true),
                Node::None => (0, false),
                Node::Node { ans, left, right } => {
                    if let Some(v) = ans.as_ref() {
                        *v
                    } else {
                        let (left_ans, left_full) = left.get_ans();
                        let (right_ans, right_full) = right.get_ans();
                        let cur_full = left_full && right_full;
                        let cur_ans = if left_full || right_full {
                            left_ans + right_ans
                        } else {
                            left_ans.max(right_ans)
                        };
                        *ans = Some((cur_ans, cur_full));
                        (cur_ans, cur_full)
                    }
                }
            }
        }

        fn add(&mut self, val: usize, bit: usize) {
            match self {
                Node::Leaf { .. } => unreachable!(),
                Node::None => {
                    *self = Node::Node {
                        ans: None,
                        left: Box::new(Node::None),
                        right: Box::new(Node::None),
                    };
                }
                _ => {}
            }
            match self {
                Node::Leaf { .. } => unreachable!(),
                Node::None => unreachable!(),
                Node::Node {
                    left, right, ans, ..
                } => {
                    *ans = None;
                    let select = if val.is_set(bit) { right } else { left };
                    if bit == 0 {
                        match select.deref_mut() {
                            Node::None => *select.deref_mut() = Node::Leaf { qty: 1 },
                            Node::Leaf { qty } => *qty += 1,
                            Node::Node { .. } => unreachable!(),
                        }
                    } else {
                        select.add(val, bit - 1);
                    }
                }
            }
        }

        fn remove(&mut self, val: usize, bit: usize) {
            match self {
                Node::Leaf { .. } => unreachable!(),
                Node::None => unreachable!(),
                Node::Node {
                    left, right, ans, ..
                } => {
                    *ans = None;
                    let other_empty = if val.is_set(bit) {
                        matches!(left.as_ref(), Node::None)
                    } else {
                        matches!(right.as_ref(), Node::None)
                    };
                    let select = if val.is_set(bit) { right } else { left };
                    if bit == 0 {
                        match select.deref_mut() {
                            Node::Leaf { qty } => {
                                *qty -= 1;
                                if *qty == 0 {
                                    *select.deref_mut() = Node::None;
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        select.remove(val, bit - 1);
                    }
                    if other_empty && matches!(select.as_ref(), Node::None) {
                        *self = Node::None;
                    }
                }
            }
        }
    }

    let mut root = Node::None;
    const MAX: usize = 29;
    for v in a.copy_iter() {
        root.add(v, MAX);
    }
    out.print_line(root.get_ans().0);
    for _ in 0..q {
        let at = input.read_size() - 1;
        let val = input.read_size();
        root.remove(a[at], MAX);
        a[at] = val;
        root.add(a[at], MAX);
        out.print_line(root.get_ans().0);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
