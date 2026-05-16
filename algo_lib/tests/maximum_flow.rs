//{"name":"Maximum Flow","group":"Kattis","url":"https://open.kattis.com/problems/maxflow","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5 0 3\n0 1 10\n1 2 1\n1 3 1\n0 2 1\n2 3 10\n","output":"4 3 5\n0 1 2\n0 2 1\n1 2 1\n1 3 1\n2 3 2\n"},{"input":"2 1 0 1\n0 1 100000\n","output":"2 100000 1\n0 1 100000\n"},{"input":"2 1 1 0\n0 1 100000\n","output":"2 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MaximumFlow"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::max_flow::MaxFlow;
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
    let s = input.read_size();
    let t = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m);

    let mut graph = Graph::new_linked(n).do_with(|graph| {
        for (u, v, w) in edges {
            graph.add_edge(FlowEdge::with_payload(u, v, w, true));
        }
    });
    let flow = graph.max_flow(s, t);
    let edges = Vec::with_capacity(m).do_with(|edges| {
        for i in 0..n {
            for e in graph.adj(i).iter() {
                if *e.payload() && e.flow(&graph) > 0 {
                    edges.push((i, e.to(), e.flow(&graph)));
                }
            }
        }
    });
    out.print_line((n, flow, edges.len()));
    out.print_per_line(&edges);
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

    fn interact(
        mut sol_input: Input,
        mut sol_output: Output,
        mut input: Input,
    ) -> Result<(), String> {
        Ok(())
    }

    fn check(
        mut input: Input,
        mut expected: Option<Input>,
        mut output: Input,
    ) -> Result<Option<i64>, String> {
        let n = input.read_size();
        let m = input.read_size();
        let s = input.read_size();
        let t = input.read_size();
        let mut caps: std::collections::HashMap<(usize, usize), i64> =
            std::collections::HashMap::new();
        for _ in 0..m {
            let u = input.read_size();
            let v = input.read_size();
            let c: i64 = input.read();
            *caps.entry((u, v)).or_insert(0) += c;
        }

        let our_n = output.read_size();
        let our_flow: i64 = output.read();
        let our_m = output.read_size();

        if our_n != n {
            return Err(format!("Output n: expected {}, got {}", n, our_n));
        }
        if let Some(exp) = expected.as_mut() {
            let _ = exp.read_size();
            let exp_flow: i64 = exp.read();
            let _ = exp.read_size();
            if our_flow != exp_flow {
                return Err(format!(
                    "Flow value mismatch: expected {}, got {}",
                    exp_flow, our_flow
                ));
            }
        }

        let mut balance = vec![0i64; n];
        for _ in 0..our_m {
            let u = output.read_size();
            let v = output.read_size();
            let f: i64 = output.read();
            if f < 0 {
                return Err(format!("Edge ({}, {}) has negative flow {}", u, v, f));
            }
            let cap = caps.get(&(u, v)).copied().unwrap_or(0);
            if cap == 0 {
                return Err(format!("Edge ({}, {}) is not in input", u, v));
            }
            if f > cap {
                return Err(format!(
                    "Edge ({}, {}): flow {} exceeds capacity {}",
                    u, v, f, cap
                ));
            }
            balance[u] -= f;
            balance[v] += f;
        }

        for v in 0..n {
            if v == s || v == t {
                continue;
            }
            if balance[v] != 0 {
                return Err(format!("Vertex {} flow imbalance: {}", v, balance[v]));
            }
        }
        if balance[s] != -our_flow {
            return Err(format!(
                "Source net out is {}, expected {}",
                -balance[s], our_flow
            ));
        }
        if balance[t] != our_flow {
            return Err(format!(
                "Sink net in is {}, expected {}",
                balance[t], our_flow
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
        let path = "./maximum_flow";
        let tl = 2000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Max test", true, MaxTest);
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn maximum_flow() {
    assert!(tester::run_tests());
}
