//{"name":"D: Vacation","group":"Facebook Coding Competitions - Facebook Hacker Cup 2021 Final Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2021/final-round/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n4 90\n10 20 30 40\n1 2 2\n4 91\n10 20 30 40\n1 2 2\n4 101\n10 20 30 40\n1 2 2\n5 222\n1 1 100 100 100\n1 1 2 2\n8 35\n3 5 7 2 4 8 1 6\n1 2 1 4 4 1 4\n8 36\n3 5 7 2 4 8 1 6\n1 2 1 4 4 1 4\n","output":"Case #1: 0\nCase #2: 1\nCase #3: -1\nCase #4: 1\nCase #5: 1\nCase #6: 2\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"vacation_.*input[.]txt"},"output":{"type":"file","fileName":"vacation_output.txt","pattern":null},"languages":{"java":{"taskClass":"DVacation"}}}

use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::collections::iter_ext::IterOrdExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        k: u64,
        c: Vec<u64>,
        p: Vec<usize>,
        ans: i32,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.k = input.read();
            self.c = input.read_vec(self.n);
            self.p = input.read_vec(self.n - 1).dec_by_one();
        }

        fn solve(&mut self) {
            let mut graph = Graph::new(self.n);
            let n = self.n;
            for i in 1..n {
                graph.add_edge(i, BiEdge::new(self.p[i - 1]));
            }
            let mut dist = vec![0u64; n];
            let mut dfs = RecursiveFunction2::new(|f, vert, prev| {
                dist[vert] = dist[prev] + self.c[vert];
                for e in graph[vert].iter() {
                    if e.to() != prev {
                        f.call(e.to(), vert);
                    }
                }
            });
            dfs.call(0, 0);
            let root = dist.iter().max_position().unwrap();
            let mut prev = vec![0usize; n];
            let mut next = vec![0usize; n];
            let mut dfs = RecursiveFunction2::new(|f, vert, p| {
                dist[vert] = 0;
                prev[vert] = p;
                next[vert] = vert;
                for e in graph[vert].iter() {
                    if e.to() != p {
                        let call = f.call(e.to(), vert);
                        if call > dist[vert] {
                            dist[vert] = call;
                            next[vert] = e.to();
                        }
                    }
                }
                dist[vert] += self.c[vert];
                dist[vert]
            });
            dfs.call(root, root);
            let mut heap = IndexedHeap::new(n);
            for e in graph[root].iter() {
                heap.add_or_adjust(e.to(), -(dist[e.to()] as i64));
            }
            let mut cur = self.c[root] as i64;
            let mut ans = 0usize;
            while cur < self.k as i64 && !heap.is_empty() {
                ans += 1;
                let mut c = Vec::new();
                for _ in 0..2 {
                    if !heap.is_empty() {
                        c.push(heap.pop());
                    }
                }
                for (mut vert, c) in c {
                    cur -= c;
                    while next[vert] != vert {
                        for e in graph[vert].iter() {
                            if e.to() != prev[vert] && e.to() != next[vert] {
                                heap.add_or_adjust(e.to(), -(dist[e.to()] as i64));
                            }
                        }
                        vert = next[vert];
                    }
                }
            }
            if cur >= self.k as i64 {
                self.ans = (ans.max(1) as i32) - 1;
            } else {
                self.ans = -1;
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}: {}", test_case, self.ans));
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
