use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    #[derive(Default, Clone)]
    struct Node {
        sum: i32,
        rtl_min: i32,
        ltr_min: i32,
        len: usize,
        hide: bool,
    }

    impl Node {
        fn sum(&self) -> i32 {
            if self.hide {
                0
            } else {
                self.sum
            }
        }

        fn rtl_min(&self) -> i32 {
            if self.hide {
                0
            } else {
                self.rtl_min
            }
        }

        fn ltr_min(&self) -> i32 {
            if self.hide {
                0
            } else {
                self.ltr_min
            }
        }

        fn len(&self) -> usize {
            if self.hide {
                0
            } else {
                self.len
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.sum = left_val.sum() + right_val.sum();
            self.ltr_min = left_val.ltr_min().min(left_val.sum() + right_val.ltr_min());
            self.rtl_min = right_val
                .rtl_min()
                .min(right_val.sum() + left_val.rtl_min());
            self.len = left_val.len() + right_val.len();
        }
    }

    let mut st = SegmentTree::with_gen(n, |_| Node {
        sum: -1,
        rtl_min: -1,
        ltr_min: -1,
        len: 1,
        hide: false,
    });

    let q = p.inv();
    let mut ans = 0;
    for i in q {
        let mut max = -st.query(..i).rtl_min.min(0);
        if i != n - 1 {
            st.for_each_mut(..=i, |_, node| node.hide = true);
            let (sum, len) = st.binary_search(
                |left, right| {
                    if left.sum() + right.ltr_min() <= max {
                        max -= left.sum();
                        ans += left.len();
                        Direction::Right
                    } else {
                        if left.hide {
                            max = i32::MIN / 2
                        }
                        Direction::Left
                    }
                },
                |node, _| (node.sum(), node.len()),
            );
            if sum <= max {
                ans += len;
            }
            st.for_each_mut(..=i, |_, node| node.hide = false);
        }
        st.point_update(
            i,
            Node {
                sum: 1,
                rtl_min: 1,
                ltr_min: 1,
                len: 1,
                hide: false,
            },
        );
    }
    out.print_line(ans);

    /*struct Node {
        val: i32,
        sum: i32,
        rtl_min: i32,
        ltr_min: i32,
        len: usize,
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            let self_left = left.map_or(0, |x| x.sum) + self.val;
            self.sum = self_left + right.map_or(0, |x| x.sum);
            self.ltr_min = self_left;
            if let Some(left) = left {
                self.ltr_min.minim(left.ltr_min);
            }
            if let Some(right) = right {
                self.ltr_min.minim(self_left + right.ltr_min);
            }
            let self_right = right.map_or(0, |x| x.sum) + self.val;
            self.rtl_min = self_right;
            if let Some(right) = right {
                self.rtl_min.minim(right.rtl_min);
            }
            if let Some(left) = left {
                self.rtl_min.minim(self_right + left.rtl_min);
            }
            self.len = 1 + left.map_or(0, |x| x.len) + right.map_or(0, |x| x.len);
        }
    }

    let mut tree = Tree::with_gen(n, |_| Node {
        val: -1,
        sum: -1,
        rtl_min: -1,
        ltr_min: -1,
        len: 1,
    });

    let q = p.inv();
    let mut ans = 0;
    for i in q {
        let (mut left, right) = tree.split_at(i);
        let mut right = right.split_at(1).1;
        let mut max = -left.payload().map_or(0, |x| x.rtl_min).min(0);
        right.binary_search(|node, left, right| {
            let left_self = left.map_or(0, |x| x.sum) + node.val;
            if let Some(right) = right {
                if left_self + right.ltr_min <= max {
                    max -= left_self;
                    ans += 1 + left.map_or(0, |x| x.len);
                    return Some(Direction::Right);
                }
            }
            if left_self <= max {
                ans += 1 + left.map_or(0, |x| x.len);
                return None;
            }
            Some(Direction::Left)
        });
        tree = Tree::merge_three(
            left,
            Tree::single(Node {
                val: 1,
                sum: 1,
                rtl_min: 1,
                ltr_min: 1,
                len: 1,
            }),
            right,
        );
    }
    out.print_line(ans);*/
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
