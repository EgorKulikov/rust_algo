//{"name":"task_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, usize)>(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, t) in edges.copy_iter() {
        graph.add_edge(BiEdge::with_payload(u, v, t));
    }
    let mut deg = Arr2d::new(n, 2, 0);
    for (u, v, t) in edges.copy_iter() {
        deg[(u, t)] += 1;
        deg[(v, t)] += 1;
    }
    let mut dist = Arr2d::new(n, 2, None);
    let mut cur = Vec::new();
    for (i, j) in deg.indices() {
        if deg[(i, j)] == 0 {
            dist[(i, j)] = Some(1);
            cur.push((i, j));
        }
    }
    let mut val = 2;
    while !cur.is_empty() {
        let mut next = Vec::new();
        for (v, t) in cur {
            for e in &graph[v] {
                if e.payload() != &t {
                    let to = e.to();
                    if dist[(to, 1 - t)].is_none() {
                        deg[(to, 1 - t)] -= 1;
                        if deg[(to, 1 - t)] == 0 {
                            dist[(to, 1 - t)] = Some(val);
                            next.push((to, 1 - t));
                        }
                    }
                }
            }
        }
        cur = next;
        val += 1;
    }
    let mut ans = None;
    for t in 0..2 {
        if let Some(d) = dist[(0, t)] {
            ans.minim(d);
        }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
