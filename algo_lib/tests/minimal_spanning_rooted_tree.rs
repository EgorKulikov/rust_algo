//{"name":"Minimal Spanning Rooted Tree","group":"Manual","url":"https://judge.yosupo.jp/problem/directedmst","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::minimal_spanning_rooted_tree::MinimalSpanningRootedTree;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::LegacyTestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let r = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m);

    let graph = Graph::new(n, m).do_with(|g| {
        for (u, v, w) in edges {
            g.add_edge(WeightedEdge::new(u, v, w));
        }
    });
    match graph.minimal_spanning_rooted_tree(r) {
        None => out.print_line(-1),
        Some(tree) => {
            let weight: i64 = graph.minimal_spanning_rooted_tree_weight(r).unwrap();
            let mut parent: Vec<usize> = (0..n).collect();
            for u in 0..n {
                for e in tree.adj(u).iter() {
                    parent[e.to()] = u;
                }
            }
            out.print_line(weight);
            out.print_line(parent);
        }
    }
}

pub static TEST_TYPE: LegacyTestType = LegacyTestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        LegacyTestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        LegacyTestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        LegacyTestType::MultiEof => {
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
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use tester::classic::default_checker;
    use tester::interactive::std_interactor;
    use tester::test_set::GeneratedTestSet;
    use tester::Tester;

    const PRINT_LIMIT: usize = 1000;

    // Any minimal arborescence is accepted: the checker verifies that the
    // parent array is a tree rooted at r whose weight matches both the
    // reported and the expected total.
    fn check(
        mut input: Input,
        expected: Option<Input>,
        mut output: Input,
    ) -> Result<Option<i64>, String> {
        let n = input.read_size();
        let m = input.read_size();
        let r = input.read_size();
        let mut min_edge = vec![vec![None::<i64>; n]; n];
        for _ in 0..m {
            let (u, v, w): (usize, usize, i64) = input.read();
            if min_edge[u][v].is_none_or(|c| w < c) {
                min_edge[u][v] = Some(w);
            }
        }
        let expected_weight: i64 = expected.unwrap().read();
        let weight: i64 = output.read();
        if weight != expected_weight {
            return Err(format!(
                "Expected weight {}, got {}",
                expected_weight, weight
            ));
        }
        if weight == -1 {
            return Ok(None);
        }
        let parent: Vec<usize> = output.read_vec(n);
        if parent[r] != r {
            return Err(format!("parent[{}] = {}, expected root", r, parent[r]));
        }
        let mut total = 0;
        for v in 0..n {
            if v == r {
                continue;
            }
            match min_edge[parent[v]][v] {
                None => return Err(format!("No edge {} -> {}", parent[v], v)),
                Some(w) => total += w,
            }
            let mut cur = v;
            for steps in 0.. {
                if cur == r {
                    break;
                }
                if steps > n {
                    return Err(format!("Vertex {} does not reach the root", v));
                }
                cur = parent[cur];
            }
        }
        if total != weight {
            return Err(format!(
                "Parent array weight {} differs from reported {}",
                total, weight
            ));
        }
        Ok(None)
    }

    struct StressTest;

    impl GeneratedTestSet for StressTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
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
        let path = "./minimal_spanning_rooted_tree";
        let tl = 2000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        passed
    }
}

#[test]
fn minimal_spanning_rooted_tree() {
    assert!(tester::run_tests());
}

mod stress {
    use algo_lib::graph::edges::edge_trait::EdgeTrait;
    use algo_lib::graph::edges::weighted_edge::WeightedEdge;
    use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
    use algo_lib::graph::minimal_spanning_rooted_tree::MinimalSpanningRootedTree;
    use algo_lib::graph::Graph;
    use algo_lib::misc::extensions::do_with::DoWith;
    use algo_lib::misc::random::{Random, RandomTrait};

    fn brute(n: usize, root: usize, edges: &[(usize, usize, i64)]) -> Option<i64> {
        let mut min_edge = vec![vec![None::<i64>; n]; n];
        for &(u, v, w) in edges {
            if u != v && min_edge[u][v].is_none_or(|c| w < c) {
                min_edge[u][v] = Some(w);
            }
        }
        let mut parent: Vec<usize> = (0..n).collect();
        let mut best = None;
        rec(0, n, root, &min_edge, &mut parent, &mut best);
        best
    }

    fn rec(
        v: usize,
        n: usize,
        root: usize,
        min_edge: &[Vec<Option<i64>>],
        parent: &mut Vec<usize>,
        best: &mut Option<i64>,
    ) {
        if v == n {
            let mut total = 0;
            for i in 0..n {
                if i == root {
                    continue;
                }
                total += min_edge[parent[i]][i].unwrap();
                let mut cur = i;
                let mut steps = 0;
                while cur != root {
                    cur = parent[cur];
                    steps += 1;
                    if steps > n {
                        return;
                    }
                }
            }
            if best.is_none_or(|b| total < b) {
                *best = Some(total);
            }
            return;
        }
        if v == root {
            rec(v + 1, n, root, min_edge, parent, best);
            return;
        }
        for p in 0..n {
            if p != v && min_edge[p][v].is_some() {
                parent[v] = p;
                rec(v + 1, n, root, min_edge, parent, best);
            }
        }
    }

    #[test]
    fn minimal_spanning_rooted_tree_stress() {
        let mut rng = Random::new_with_seed(566);
        for _ in 0..300 {
            let n = 2 + rng.gen_bound(5usize);
            let m = 1 + rng.gen_bound(3 * n);
            let root = rng.gen_bound(n);
            let edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| {
                    (
                        rng.gen_bound(n),
                        rng.gen_bound(n),
                        rng.gen_bound(10u64) as i64,
                    )
                })
                .collect();
            let graph = Graph::new(n, m).do_with(|g| {
                for &(u, v, w) in &edges {
                    g.add_edge(WeightedEdge::new(u, v, w));
                }
            });
            let expected = brute(n, root, &edges);
            let weight: Option<i64> = graph.minimal_spanning_rooted_tree_weight(root);
            assert_eq!(
                weight, expected,
                "weight mismatch: n={n} root={root} edges={edges:?}"
            );
            let tree = graph.minimal_spanning_rooted_tree(root);
            assert_eq!(
                tree.is_some(),
                expected.is_some(),
                "existence mismatch: n={n} root={root} edges={edges:?}"
            );
            if let Some(tree) = tree {
                let mut total = 0;
                let mut parent: Vec<usize> = (0..n).collect();
                for u in 0..n {
                    for e in tree.adj(u).iter() {
                        parent[e.to()] = u;
                        total += e.weight();
                    }
                }
                assert_eq!(
                    Some(total),
                    expected,
                    "tree weight mismatch: n={n} root={root} edges={edges:?}"
                );
                for i in 0..n {
                    let mut cur = i;
                    let mut steps = 0;
                    while cur != root {
                        cur = parent[cur];
                        steps += 1;
                        assert!(steps <= n, "not a tree: n={n} root={root} edges={edges:?}");
                    }
                }
            }
        }
    }
}
