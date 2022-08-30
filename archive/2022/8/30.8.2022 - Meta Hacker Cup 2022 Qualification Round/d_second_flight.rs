//{"name":"D: Second Flight","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n4 5 6\n1 2 10\n1 3 5\n2 3 15\n2 4 10\n3 4 7\n1 2\n1 3\n2 3\n2 4\n3 4\n4 1\n4 3 6\n1 2 10\n2 3 20\n3 1 30\n1 2\n1 3\n1 4\n2 3\n2 4\n3 4\n4 3 6\n1 2 20\n2 3 10\n3 4 30\n1 2\n1 3\n1 4\n2 3\n2 4\n3 4\n","output":"Case #1: 25 20 42 27 24 15\nCase #2: 40 70 0 50 0 0\nCase #3: 40 10 0 20 10 60\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_flight_.*input[.]txt"},"output":{"type":"file","fileName":"second_flight_output.txt","pattern":null},"languages":{"java":{"taskClass":"DSecondFlight"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        m: usize,
        q: usize,
        edges: Vec<(usize, usize, i64)>,
        requests: Vec<(usize, usize)>,
        f: Vec<i64>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.m = input.read();
            self.q = input.read();
            self.edges = input.read_vec(self.m);
            self.requests = input.read_vec(self.q).dec_by_one();
        }

        fn solve(&mut self) {
            const BUBEN: usize = 500;

            let mut graph = Graph::new(self.n);
            let mut edges = DefaultMap::new();
            let mut ans = DefaultMap::new();
            for &(a, b, c) in &self.edges {
                edges[(a - 1, b - 1)] = c;
                edges[(b - 1, a - 1)] = c;
                ans[(a - 1, b - 1)] = 2 * c;
                ans[(b - 1, a - 1)] = 2 * c;
                graph.add_edge(a - 1, BiWeightedEdge::new(b - 1, c));
            }
            let mut hubs = Vec::new();
            for i in 0..self.n {
                if graph[i].len() <= BUBEN {
                    for e in &graph[i] {
                        for f in &graph[i] {
                            if f.to() == e.to() {
                                break;
                            }
                            ans[(e.to(), f.to())] += e.weight().min(f.weight());
                            ans[(f.to(), e.to())] += e.weight().min(f.weight());
                        }
                    }
                } else {
                    hubs.push(i);
                }
            }
            for &(x, y) in &self.requests {
                let mut res = ans[(x, y)];
                for &i in &hubs {
                    res += edges[(x, i)].min(edges[(y, i)]);
                }
                self.f.push(res);
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case), self.f);
        }
    }

    run_parallel::<Job>(input);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
