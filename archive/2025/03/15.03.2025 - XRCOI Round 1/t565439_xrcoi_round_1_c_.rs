//{"name":"T565439 [XRCOI Round 1] C. 草萤有耀终非火","group":"Luogu","url":"https://www.luogu.com.cn/problem/T565439?contestId=221170","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n5 5 7\n3 1\n2 2\n5 4\n4 4\n3 4\n1 4\n3 5\n","output":"4\n"},{"input":"1\n5 5 7\n1 1\n3 1\n3 3\n2 3\n4 3\n2 5\n4 5\n","output":"5\n"},{"input":"1\n5 5 3\n1 1\n1 3\n1 5\n","output":"2\n"},{"input":"1\n5 5 4\n1 1\n1 3\n2 2\n5 5\n","output":"-1\n"},{"input":"1\n3 1 4\n1 1\n2 1\n1 1\n2 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let xy = input.read_size_pair_vec(k).dec();

    let graph = Graph::new(n + m).do_with(|g| {
        for (x, y) in xy {
            g.add_edge(BiEdge::new(x, y + n));
        }
    });

    let d_top = graph.edge_distances(0);
    if d_top[n] == u32::MAX || d_top[n + m - 1] == u32::MAX {
        out.print_line(-1);
        return;
    }
    let d_left = graph.edge_distances(n);
    let d_right = graph.edge_distances(n + m - 1);
    let mut ans = None;
    for i in 0..n + m {
        if d_top[i] != u32::MAX && d_left[i] != u32::MAX && d_right[i] != u32::MAX {
            let sum = d_top[i] + d_left[i] + d_right[i];
            ans.minim(sum);
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
