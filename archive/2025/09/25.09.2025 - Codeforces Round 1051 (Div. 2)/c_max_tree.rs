//{"name":"C. Max Tree","group":"Codeforces - Codeforces Round 1051 (Div. 2)","url":"https://codeforces.com/contest/2143/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 2 2 1\n2 3 3 2\n5\n1 2 1 3\n1 5 2 1\n2 4 5 7\n2 3 1 100\n5\n2 5 5 2\n3 5 4 6\n4 5 1 5\n1 5 3 5\n","output":"3 2 1\n2 3 5 4 1\n1 5 2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64, i64)>(n - 1).dec();

    let mut graph = Graph::new(n);
    let mut incoming = vec![0; n];
    for (u, v, x, y) in edges {
        if x < y {
            graph.add_edge(Edge::new(u, v));
            incoming[v] += 1;
        } else {
            graph.add_edge(Edge::new(v, u));
            incoming[u] += 1;
        }
    }
    let mut queue = Vec::new();
    for i in 0..n {
        if incoming[i] == 0 {
            queue.push(i);
        }
    }
    let mut ans = vec![0; n];
    for i in 1..=n {
        let v = queue.pop().unwrap();
        ans[v] = i;
        for e in &graph[v] {
            let to = e.to();
            incoming[to] -= 1;
            if incoming[to] == 0 {
                queue.push(to);
            }
        }
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
