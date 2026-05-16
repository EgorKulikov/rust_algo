//{"name":"C: Wiki Race","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-2/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n6\n1 1 2 2 5\n4 mozzarella cheddar gouda edam\n5 gouda gjetost mozzarella cheddar edam\n3 gruyere mozzarella edam\n1 gouda\n1 edam\n3 cheddar gruyere edam\n4\n1 2 2\n2 earth water\n2 wind wind\n2 earth fire\n2 light metal\n3\n1 2\n1 a\n1 b\n1 c\n","output":"Case #1: 3\nCase #2: 1\nCase #3: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"wiki_race_.*input[.]txt"},"output":{"type":"file","fileName":"wiki_race_output.txt","pattern":null},"languages":{"java":{"taskClass":"CWikiRace"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use std::collections::HashSet;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        p: Vec<usize>,
        s: Vec<Vec<Str<'static>>>,
        ans: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read_size();
            self.p = input.read_vec(self.n - 1).dec();
            self.s = (0..self.n)
                .map(|_| {
                    let m = input.read_size();
                    input.read_vec(m)
                })
                .collect();
        }

        fn solve(&mut self) {
            let mut graph = Graph::new(self.n);
            for i in 0..self.n - 1 {
                graph.add_edge(self.p[i], Edge::new(i + 1));
            }
            let mut dfs = RecursiveFunction::new(
                |dfs, vert: usize| -> (HashSet<Str>, Option<HashSet<Str>>) {
                    let (mut all, mut but_one) = match graph[vert].len() {
                        0 => (HashSet::new(), None),
                        1 => dfs.call(graph[vert][0].to()),
                        _ => {
                            let (mut all, mut but_one) = dfs.call(graph[vert][0].to());
                            for e in graph[vert].iter().skip(1) {
                                let (mut call_all, mut call_but_one) = dfs.call(e.to());
                                if all.len() + but_one.as_ref().map(|v| v.len()).unwrap_or(0)
                                    < call_all.len()
                                        + call_but_one.as_ref().map(|v| v.len()).unwrap_or(0)
                                {
                                    std::mem::swap(&mut all, &mut call_all);
                                    std::mem::swap(&mut but_one, &mut call_but_one);
                                }
                                let mut new_all = HashSet::new();
                                let mut new_but_one = HashSet::new();
                                for s in call_all {
                                    if all.contains(&s) {
                                        all.remove(&s);
                                        new_all.insert(s);
                                    } else if but_one.is_none()
                                        || but_one.as_ref().unwrap().contains(&s)
                                    {
                                        new_but_one.insert(s);
                                    }
                                }
                                if call_but_one.is_none() {
                                    for s in new_but_one {
                                        all.insert(s);
                                    }
                                    (all, but_one) = (new_all, Some(all));
                                } else {
                                    for s in call_but_one.unwrap() {
                                        if all.contains(&s) {
                                            new_but_one.insert(s);
                                        }
                                    }
                                    (all, but_one) = (new_all, Some(new_but_one));
                                }
                            }
                            (all, but_one)
                        }
                    };
                    for s in &self.s[vert] {
                        if but_one.is_none() || but_one.as_ref().unwrap().contains(s) {
                            all.insert(s.clone());
                            if let Some(but_one) = but_one.as_mut() {
                                but_one.remove(s);
                            }
                        }
                    }
                    (all, but_one)
                },
            );
            self.ans = dfs.call(0).0.len();
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.ans));
        }
    }

    run_parallel::<Job>(input, output);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    solve(&mut input, &mut output, &pre_calc);
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
