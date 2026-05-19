//{"name":"M. Rounddog","group":"Universal Cup - GP of Wulin","url":"https://contest.ucup.ac/contest/3749/problem/18131","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n1 2\n3\n1 3 2\n4\n3 4 1 2\n5\n2 5 1 3 4\n6\n3 6 2 1 4 5\n","output":"0 0\n0 0 0\n0 0 1 0\n0 0 1 0 0\n0 0 1 2 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    #[derive(Clone, Default)]
    struct Node {
        // len: Arr2d<Option<usize>>,
        inc: usize,
        dec: usize,
        mixed: usize,
        both: usize,
        total: usize,
    }

    // impl Default for Node {
    //     fn default() -> Self {
    //         Self {
    //             len: Arr2d::new(2, 2, None),
    //         }
    //     }
    // }

    impl Node {
        fn int_update(&mut self) {
            if self.inc == 1 && self.dec == 1 {
                self.both = 1;
            } else {
                self.both = 0;
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            /*self.len.fill(None);
            for (a, b) in left_val.len.indices() {
                for (c, d) in right_val.len.indices() {
                    if let Some(len1) = left_val.len[(a, b)] {
                        self.len[(a, b)].maxim(len1);
                        if let Some(len2) = right_val.len[(c, d)] {
                            let mut len = len1 + len2;
                            if b == c {
                                len = len.saturating_sub(1);
                            }
                            self.len[(a, d)].maxim(len);
                        }
                    }
                    if let Some(len2) = right_val.len[(c, d)] {
                        self.len[(c, d)].maxim(len2);
                    }
                }
            }*/
            self.inc = left_val.inc + right_val.inc;
            self.dec = left_val.dec + right_val.dec;
            self.mixed = 0;
            if left_val.mixed > 0 {
                self.mixed.maxim(left_val.mixed + right_val.inc);
            }
            if right_val.mixed > 0 {
                self.mixed.maxim(left_val.dec + right_val.mixed);
            }
            if left_val.dec != 0 && right_val.inc != 0 {
                self.mixed.maxim(left_val.dec + right_val.inc - 1);
            }
            self.both = left_val.both + right_val.both;
            // self.mixed = (left_val.mixed + right_val.inc)
            //     .max(left_val.dec + right_val.mixed)
            //     .max(left_val.dec + right_val.inc);
            self.total = left_val.total + right_val.total + left_val.dec * right_val.inc;
        }
    }

    let mut st = SegmentTree::<Node>::new(n);
    let mut lis = Vec::new();
    let mut lds = Vec::new();
    let mut ans = Vec::with_capacity(n);
    for i in p {
        let p_lis = lis.lower_bound(&i);
        if p_lis < lis.len() {
            st.for_each_mut(lis[p_lis]..=lis[p_lis], |_, node| {
                node.inc = 0;
                node.int_update();
            });
            lis[p_lis] = i;
        } else {
            lis.push(i);
        }
        st.for_each_mut(i..=i, |_, node| {
            node.inc = 1;
            node.int_update();
        });
        let p_lds = lds.lower_bound(&Reverse(i));
        if p_lds < lds.len() {
            st.for_each_mut(lds[p_lds].0..=lds[p_lds].0, |_, node| {
                node.dec = 0;
                node.int_update();
            });
            lds[p_lds] = Reverse(i);
        } else {
            lds.push(Reverse(i));
        }
        st.for_each_mut(i..=i, |_, node| {
            node.dec = 1;
            node.int_update();
        });
        // let res = st.query(..).mixed;
        let node = st.query(lds[Back(0)].0..=lis[Back(0)]);
        // ans.push((node.inc + node.dec).saturating_sub(3));
        ans.push(node.total);
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
