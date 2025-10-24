//{"name":"D. Max Mex","group":"Codeforces - GoForGold Long Challenge - May 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/611602/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n3 4\n1 2 3\n1 3 2\n2 0\n1 1 2\n2 2\n5 3\n1 3 5 7 9\n2 1\n1 4 1\n2 2\n","output":"3\n4\n4\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_size_vec(n).dec();

    #[derive(Clone)]
    struct Node {
        vals: usize,
        num_empty: usize,
    }

    impl Default for Node {
        fn default() -> Self {
            Node {
                vals: 0,
                num_empty: 1,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.num_empty = left_val.num_empty + right_val.num_empty;
        }
    }

    let mut st = SegmentTree::<Node>::new(n);

    for i in a.copy_iter() {
        st.for_each_mut(i..=i, |_, node| {
            node.vals += 1;
            node.num_empty = 0;
        });
    }

    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let i = input.read_size() - 1;
                let x = input.read_size() - 1;
                st.for_each_mut(a[i]..=a[i], |_, node| {
                    node.vals -= 1;
                    if node.vals == 0 {
                        node.num_empty = 1;
                    }
                });
                a[i] = x;
                st.for_each_mut(a[i]..=a[i], |_, node| {
                    node.vals += 1;
                    node.num_empty = 0;
                });
            }
            2 => {
                let k = input.read_size();
                if st.query(..).num_empty <= k {
                    out.print_line(n + 1);
                } else {
                    let mut rem = k;
                    out.print_line(st.binary_search(
                        |left, _| {
                            if left.num_empty > rem {
                                Direction::Left
                            } else {
                                rem -= left.num_empty;
                                Direction::Right
                            }
                        },
                        |_, at| at + 1,
                    ));
                }
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
