//{"name":"E - Greedy Takahashi 2","group":"AtCoder - AtCoder Regular Contest 211 (Div. 2)","url":"https://atcoder.jp/contests/arc211/tasks/arc211_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n4 2 3 1\n4\n4 1 3 2\n11\n11 8 9 6 7 10 3 4 5 1 2\n","output":"Yes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::{BTreeSet, VecDeque};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    if p[0] != n - 1 {
        out.print_line(false);
        return;
    }
    let mut set = (0..n - 1).collect::<BTreeSet<_>>();
    let mut min = n - 1;
    let mut graph = Graph::new(n);
    #[derive(Default, Copy, Clone)]
    struct Node {
        last: usize,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.last = left_val.last.max(right_val.last);
        }
    }
    let mut st = SegmentTree::<Node>::new(n);
    for i in 1..n {
        if min > p[i] {
            min = p[i];
        } else {
            let expected = set.next(&min).copied();
            if expected != Some(p[i]) {
                out.print_line(false);
                return;
            }
            set.remove(&p[i]);
        }
        let parent = st.query(p[i]..).last;
        graph.add_edge(Edge::new(p[parent], p[i]));
        set.remove(&p[i]);
        st.point_through_update(p[i], |node| node.last = i);
    }
    let mut good = true;
    let mut dfs = RecursiveFunction::new(|f, vert: usize| -> VecDeque<usize> {
        let mut calls = Vec::new();
        for e in &graph[vert] {
            let call = f.call(e.to());
            if !calls.is_empty() && calls[Back(0)] > call {
                good = false;
            }
            calls.push(call);
        }
        let mut res = VecDeque::new();
        res.push_back(graph[vert].len());
        for mut call in calls {
            if res.len() < call.len() {
                for i in res.iter_rev() {
                    call.push_front(i);
                }
                res = call;
            } else {
                res.extend(call.into_iter());
            }
        }
        res
    });
    dfs.call(n - 1);
    out.print_line(good);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
