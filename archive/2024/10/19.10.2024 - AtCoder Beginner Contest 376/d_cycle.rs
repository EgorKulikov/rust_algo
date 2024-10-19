//{"name":"D - Cycle","group":"AtCoder - AtCoder Beginner Contest 376","url":"https://atcoder.jp/contests/abc376/tasks/abc376_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 2\n2 3\n3 1\n","output":"3\n"},{"input":"3 2\n1 2\n2 3\n","output":"-1\n"},{"input":"6 9\n6 1\n1 5\n2 6\n2 1\n3 6\n4 2\n6 4\n3 5\n5 4\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCycle"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let direct = Graph::from_edges(n, &edges);
    let reverse = Graph::new(n).do_with(|g| {
        for (u, v) in edges {
            g.add_edge(Edge::new(v, u));
        }
    });

    let dd = direct.edge_distances(0);
    let rd = reverse.edge_distances(0);
    let mut ans = None;
    for i in 1..n {
        if dd[i] != u32::MAX && rd[i] != u32::MAX {
            ans.minim(dd[i] + rd[i]);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
