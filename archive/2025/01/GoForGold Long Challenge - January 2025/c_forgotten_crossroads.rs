//{"name":"C. Forgotten Crossroads","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/C","interactive":false,"timeLimit":5000,"tests":[{"input":"5 4\n1 2\n2 3\n2 4\n4 5\n2 1\n2 5\n1 2\n2 5\n","output":"0\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::hl_decomposition::{HLDecomposition, HLDecompositionTrait};
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let HLDecomposition { paths, id, pos } = graph.hl_decomposition();
    let lca = graph.lca();

    #[derive(Clone)]
    struct Node {
        left: usize,
        right: usize,
        from_left: usize,
        from_right: usize,
    }
    const INF: usize = usize::MAX / 2;
    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Self {
                left,
                right: right - 1,
                from_left: INF,
                from_right: INF,
            }
        }

        fn accumulate(&mut self, value: &Self) {
            self.from_left
                .minim(value.from_left + self.left - value.left);
            self.from_right
                .minim(value.from_right + value.right - self.right);
        }
    }
    let mut trees =
        Vec::with_gen_prefix(paths.len(), |i, _| SegmentTree::<Node>::new(paths[i].len()));
    trees[0].update(
        ..,
        &Node {
            left: 0,
            right: paths[0].len() - 1,
            from_left: 0,
            from_right: INF,
        },
    );

    for _ in 0..m {
        let t = input.read_int();
        let mut v = input.read_size() - 1;
        match t {
            1 => {
                let mut add = 0;
                loop {
                    let id = id[v];
                    let pos = pos[v];
                    trees[id].update(
                        ..=pos,
                        &Node {
                            left: 0,
                            right: pos,
                            from_left: INF,
                            from_right: add,
                        },
                    );
                    trees[id].update(
                        pos..,
                        &Node {
                            left: pos,
                            right: paths[id].len() - 1,
                            from_left: add,
                            from_right: INF,
                        },
                    );
                    add += pos + 1;
                    v = paths[id][0];
                    if v == 0 {
                        break;
                    }
                    v = lca.parent(v).unwrap();
                }
            }
            2 => {
                let mut ans = INF;
                let mut add = 0;
                loop {
                    let id = id[v];
                    let pos = pos[v];
                    let call: Node = trees[id].query(pos..=pos);
                    ans.minim(add + call.from_left);
                    ans.minim(add + call.from_right);
                    add += pos + 1;
                    v = paths[id][0];
                    if v == 0 {
                        break;
                    }
                    v = lca.parent(v).unwrap();
                }
                out.print_line(ans);
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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

//START MAIN
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
//END MAIN
