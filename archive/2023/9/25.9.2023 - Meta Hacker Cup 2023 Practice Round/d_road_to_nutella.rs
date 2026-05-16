//{"name":"D: Road to Nutella","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/practice-round/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n13 15\n1 2\n2 3\n2 4\n4 5\n3 5\n5 6\n6 7\n6 8\n8 9\n8 12\n8 13\n9 10\n10 11\n11 12\n12 13\n5\n7 1\n7 8\n1 2\n1 11\n2 3\n4 3\n1 2\n2 3\n2 4\n2\n2 3\n1 4\n4 4\n1 2\n2 3\n1 3\n3 4\n2\n2 3\n1 4\n4 4\n1 2\n2 3\n1 4\n3 4\n2\n2 3\n1 4\n","output":"Case #1: 5\nCase #2: -2\nCase #3: 0\nCase #4: -2\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"road_to_nutella_.*input[.]txt"},"output":{"type":"file","fileName":"road_to_nutella_output.txt","pattern":null},"languages":{"java":{"taskClass":"DRoadToNutella"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::collections::{HashSet, VecDeque};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        m: usize,
        edges: Vec<(usize, usize)>,
        q: usize,
        queries: Vec<(usize, usize)>,
        ans: i64,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.m = input.read();
            self.edges = input.read_size_pair_vec(self.m).dec();
            self.q = input.read();
            self.queries = input.read_size_pair_vec(self.q).dec();
        }

        fn solve(&mut self) {
            let graph = Graph::from_biedges(self.n, &self.edges);
            let bridges = graph.bridges().into_iter().collect::<HashSet<_>>();
            let mut dsu = DSU::new(self.n);
            for &(u, v) in &self.edges {
                if !bridges.contains(&(u, v)) && !bridges.contains(&(v, u)) {
                    dsu.join(u, v);
                }
            }
            let mut n = 0;
            let mut id = vec![n; self.n];
            for i in 0..self.n {
                if dsu.get(i) == i {
                    id[i] = n;
                    n += 1;
                }
            }
            for i in 0..self.n {
                id[i] = id[dsu.get(i)];
            }
            let mut color = vec![0; self.n];
            let mut is_bad = BitSet::new(n);
            for i in 0..self.n {
                if color[i] != 0 {
                    continue;
                }
                let mut q = VecDeque::new();
                color[i] = 1;
                let mut bad = false;
                q.push_back(i);
                while let Some(cur) = q.pop_front() {
                    for e in &graph[cur] {
                        let to = e.to();
                        if bridges.contains(&(cur, to)) || bridges.contains(&(to, cur)) {
                            continue;
                        }
                        if color[to] == 0 {
                            color[to] = -color[cur];
                            q.push_back(to);
                        } else if color[to] != -color[cur] {
                            bad = true;
                        }
                    }
                }
                if bad {
                    is_bad.set(id[i]);
                }
            }
            let mut graph = Graph::new(n);
            for (u, v) in bridges {
                graph.add_edge(id[u], BiEdge::new(id[v]));
            }
            let mut dist = vec![n; n];
            let mut q = VecDeque::new();
            for i in 0..n {
                if is_bad[i] {
                    dist[i] = 0;
                    q.push_back(i);
                }
            }
            if q.is_empty() {
                self.ans = -self.q.into_i64();
                return;
            }
            while let Some(cur) = q.pop_front() {
                for e in &graph[cur] {
                    let to = e.to();
                    if dist[to] == n {
                        dist[to] = dist[cur] + 1;
                        q.push_back(to);
                    }
                }
            }
            let mut up = Arr2d::new(20, n, 0);
            let mut par = Arr2d::new(20, n, 0);
            let lca = graph.lca();
            for i in 0..n {
                up[(0, i)] = dist[i];
                par[(0, i)] = lca.parent(i).unwrap_or(0);
            }
            for i in 1..20 {
                for j in 0..n {
                    up[(i, j)] = up[(i - 1, j)].min(up[(i - 1, par[(i - 1, j)])]);
                    par[(i, j)] = par[(i - 1, par[(i - 1, j)])];
                }
            }
            for &(a, b) in &self.queries {
                let a = id[a];
                let b = id[b];
                let l = lca.lca(a, b);
                let mut cur = dist[l];

                for mut i in [a, b] {
                    for j in (0..20).rev() {
                        let to = par[(j, i)];
                        if lca.level(to) >= lca.level(l) {
                            cur.minim(up[(j, i)]);
                            i = to;
                        }
                    }
                }
                self.ans += cur.into_i64();
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case), self.ans);
        }
    }

    run_parallel::<Job>(input);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();
    solve(&mut input, &pre_calc);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
