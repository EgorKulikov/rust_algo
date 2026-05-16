//{"name":"M. Obliviate, Then Reincarnate","group":"Universal Cup - The 3rd Universal Cup. Stage 19: Shenyang","url":"https://contest.ucup.ac/contest/1865/problem/9810","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2 3\n1 1\n-1 3\n1\n2\n3\n","output":"Yes\nYes\nNo\n"},{"input":"3 2 3\n1 1\n-1 0\n1\n2\n3\n","output":"No\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MObliviateThenReincarnate"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::strongly_connected_components::{
    StronglyConnectedComponents, StronglyConnectedComponentsTrait,
};
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let edges = input.read_long_pair_vec(m);

    let mut graph = Graph::new(n);
    for (a, b) in edges {
        let a1 = ((a % (n as i64) + n as i64) % (n as i64)) as usize;
        let b1 = ((b % (n as i64) + n as i64) % (n as i64)) as usize;
        graph.add_edge(Edge::with_payload(a1, (a1 + b1) % n, b));
    }
    let StronglyConnectedComponents { color, condensed } = graph.strongly_connected_components();
    // let mut color = vec![0; n];
    let mut dist = vec![0; n];
    let mut good = BitSet::new(n);
    let mut done = BitSet::new(n);
    for i in 0..n {
        if !done[i] {
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, val: i64| {
                if color[vert] != color[i] {
                    return;
                }
                if done[vert] {
                    if dist[vert] != val {
                        good.set(vert);
                    }
                    return;
                }
                done.set(vert);
                dist[vert] = val;
                for e in &graph[vert] {
                    f.call(e.to(), val + e.payload());
                }
            });
            dfs.call(i, 0);
        }
    }
    let mut c_good = BitSet::new(condensed.vertex_count());
    for i in 0..n {
        if good[i] {
            c_good.set(color[i]);
        }
    }
    for i in (0..condensed.vertex_count()).rev() {
        for e in &condensed[i] {
            if c_good[e.to()] {
                c_good.set(i);
                break;
            }
        }
    }
    for _ in 0..q {
        let x = input.read_long();
        let x1 = ((x % (n as i64) + n as i64) % (n as i64)) as usize;
        out.print_line(c_good[color[x1]]);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
