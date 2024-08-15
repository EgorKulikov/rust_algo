//{"name":"G. Звонок во время пути","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"7\n5 5\n100 20 80\n1 5 30 100\n1 2 20 50\n2 3 20 50\n3 4 20 50\n4 5 20 50\n2 1\n100 50 60\n1 2 55 110\n4 4\n100 40 60\n1 2 30 100\n2 4 30 100\n1 3 20 50\n3 4 20 50\n3 3\n100 80 90\n1 2 1 10\n2 3 10 50\n1 3 20 21\n3 2\n58 55 57\n2 1 1 3\n2 3 3 4\n2 1\n12 9 10\n2 1 6 10\n5 5\n8 5 6\n2 1 1 8\n2 3 4 8\n4 2 2 4\n5 3 3 4\n4 5 2 6\n","output":"0\n-1\n60\n80\n53\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GZvonokVoVremyaPuti"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let t0 = input.read_long();
    let t1 = input.read_long();
    let t2 = input.read_long();
    let edges = input.read_vec::<(usize, usize, i64, i64)>(m).dec();

    let mut ans = None;
    let mut graph = Graph::new(n);
    for &(u, v, l1, _) in &edges {
        graph.add_edge(BiWeightedEdge::new(u, v, l1));
    }
    let dist = graph.distances_from(n - 1);
    if dist[0].unwrap().0 <= t0 - t2 {
        ans.maxim(t0 - dist[0].unwrap().0);
    }
    let mut graph = Graph::new(n);
    for &(u, v, _, l2) in &edges {
        graph.add_edge(BiWeightedEdge::new(u, v, l2));
    }
    for i in 0..n {
        let d = dist[i].unwrap().0;
        if d <= t0 - t2 {
            graph.add_edge(BiWeightedEdge::new(n - 1, i, d));
        }
    }
    let dist = graph.distances_from(n - 1);
    if dist[0].unwrap().0 <= t0 {
        ans.maxim(t0 - dist[0].unwrap().0);
    }
    let mut graph = Graph::new(n + 1);
    for &(u, v, l1, _) in &edges {
        graph.add_edge(BiWeightedEdge::new(u, v, l1));
    }
    for i in 0..n {
        let d = dist[i].unwrap().0;
        graph.add_edge(BiWeightedEdge::new(n, i, d.max(t0 - t1)));
    }
    let dist = graph.distance(n, 0).unwrap().0;
    if dist <= t0 {
        ans.maxim(t0 - dist);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
