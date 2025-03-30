//{"name":"P5 - lp0 is on fire!","group":"DMOJ - Arcadia Computing Contest 2","url":"https://dmoj.ca/problem/ahscc2p5","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 2 2\n2 4\n1 2 1\n2 3 2\n2 4 3\n3 4 1\n","output":"0000\n"},{"input":"5 7 2 5\n1 3\n3 4 5\n5 1 7\n5 4 2\n5 2 1\n1 3 10\n5 3 1\n4 2 2\n","output":"10000\n"},{"input":"5 7 4 5\n4 2 1 3\n3 4 6\n3 1 5\n4 5 8\n1 5 4\n1 2 1\n3 5 7\n2 4 3\n","output":"00110\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let t = input.read_long();
    let p = input.read_size_vec(k).dec();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut graph = Graph::new(n + 1);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let mut bad = BitSet::new(n);
    for i in p {
        bad.set(i);
    }
    for i in 0..n {
        if !bad[i] {
            graph.add_edge(BiWeightedEdge::new(n, i, 0));
        }
    }
    let dist = graph.distances_from(n);
    let mut ans = Str::from(vec![b'0'; n]);
    for i in 0..n {
        if let Some((d, ..)) = dist[i] {
            if d > t {
                ans[i] = b'1';
            }
        } else {
            ans[i] = b'1';
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
