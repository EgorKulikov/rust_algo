//{"name":"P6 - Candygrams","group":"DMOJ - UTS Open '24","url":"https://dmoj.ca/problem/utso24p6","interactive":false,"timeLimit":1000,"tests":[{"input":"7 5\n1 2\n3 1\n2 4\n4 6\n5 4\n6 7\nU 5 2\nQ 1 4\nU 7 3\nU 5 -1\nQ 6 3\n","output":"4\n12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P6Candygrams"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::gen::VecGen;
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
    let q = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let lca = graph.lca();
    let HLDecomposition { paths, id, pos } = graph.hl_decomposition();
    #[derive(Clone, Default)]
    struct Node {
        val: i64,
        delta: i64,
        len: i64,
    }
    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Self {
                len: (right - left) as i64,
                ..Default::default()
            }
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val + right_val.val;
        }

        fn accumulate(&mut self, value: &Self) {
            self.val += value.delta * self.len;
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }
    let mut st = Vec::gen(paths.len(), |i, _| SegmentTree::<Node>::new(paths[i].len()));

    let mut base = 0;

    for _ in 0..q {
        let command = input.read_char();
        match command {
            b'U' => {
                let mut x = input.read_size() - 1;
                let k = input.read_long();
                base += k * lca.level(x) as i64;
                loop {
                    let id = id[x];
                    let pos = pos[x];
                    st[id].update(
                        0..=pos,
                        &Node {
                            delta: k,
                            ..Default::default()
                        },
                    );
                    if paths[id][0] == 0 {
                        break;
                    }
                    x = lca.parent(paths[id][0]).unwrap();
                }
            }
            b'Q' => {
                let mut a = input.read_size() - 1;
                let mut b = input.read_size() - 1;
                if lca.level(a) > lca.level(b) {
                    std::mem::swap(&mut a, &mut b);
                }
                if lca.level(a) == lca.level(b) {
                    out.print_line(base);
                    continue;
                }
                let mut ans = base;
                let mut full = (lca.level(b) - lca.level(a) - 1) / 2;
                let add = lca.level(b) % 2 == lca.level(a) % 2;
                loop {
                    let id = id[b];
                    let mut pos = pos[b];
                    if pos >= full {
                        if full != 0 {
                            ans -= st[id].query(pos - full + 1..=pos).val * 2;
                            pos -= full;
                        }
                        if add {
                            ans -= st[id].query(pos..=pos).val;
                        }
                        break;
                    }
                    ans -= st[id].query(..=pos).val * 2;
                    full -= pos + 1;
                    b = lca.parent(paths[id][0]).unwrap();
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

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]
    #![allow(unused_imports)]

    use crate::{run, TASK_TYPE};
    use algo_lib::collections::dsu::DSU;
    use algo_lib::collections::vec_ext::inc_dec::IncDec;
    use algo_lib::graph::edge_distances::EdgeAlgos;
    use algo_lib::graph::edges::bi_edge::BiEdge;
    use algo_lib::graph::Graph;
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::misc::random::{Random, RandomTrait};
    use tester::classic::default_checker;
    use tester::interactive::std_interactor;
    use tester::test_set::GeneratedTestSet;
    use tester::Tester;

    const PRINT_LIMIT: usize = 1000;

    fn interact(
        mut sol_input: Input,
        mut sol_output: Output,
        mut input: Input,
    ) -> Result<(), String> {
        Ok(())
    }

    fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
        Ok(())
    }

    struct StressTest;

    impl GeneratedTestSet for StressTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let mut r = Random::new();
            let n = r.gen_range(2..=6);
            let q = r.gen_range(2..=4);
            out.print_line((n, q));
            let mut dsu = DSU::new(n);
            for _ in 1..n {
                loop {
                    let a = r.gen_range(1..=n);
                    let b = r.gen_range(1..=n);
                    if dsu.union(a - 1, b - 1) {
                        out.print_line((a, b));
                        break;
                    }
                }
            }
            let mut qty = vec![0; n + 1];
            for _ in 0..q - 1 {
                let x = r.gen_range(1..=n);
                if qty[x] != 0 && r.gen_bool() {
                    let k = r.gen_range(1..=qty[x]);
                    qty[x] -= k;
                    out.print_line(('U', x, -k));
                } else {
                    let k = r.gen_range(1..=10);
                    qty[x] += k;
                    out.print_line(('U', x, k));
                }
            }
            loop {
                let a = r.gen_range(1..=n);
                let b = r.gen_range(1..=n);
                if a != b {
                    out.print_line(('Q', a, b));
                    break;
                }
            }
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            let n = input.read_size();
            let q = input.read_size();
            let edges = input.read_size_pair_vec(n - 1).dec();
            let graph = Graph::from_biedges(n, &edges);
            let mut qty = vec![0; n];
            for _ in 0..q {
                let command = input.read_char();
                match command {
                    b'U' => {
                        let x = input.read_size() - 1;
                        let k = input.read_long();
                        qty[x] += k;
                    }
                    b'Q' => {
                        let a = input.read_size() - 1;
                        let b = input.read_size() - 1;
                        let mut graph = graph.clone();
                        graph.add_edge(BiEdge::new(a, b));
                        let d = graph.edge_distances(0);
                        let mut ans = 0;
                        for i in 0..n {
                            ans += qty[i] * d[i] as i64;
                        }
                        out.print_line(ans);
                    }
                    _ => unreachable!(),
                }
            }
            true
        }
    }

    struct MaxTest;

    impl GeneratedTestSet for MaxTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..=1
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./p6_candygrams";
        let tl = 1000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
                // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Max test", true, MaxTest);
        tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn p6_candygrams() {
    assert!(tester::run_tests());
}
