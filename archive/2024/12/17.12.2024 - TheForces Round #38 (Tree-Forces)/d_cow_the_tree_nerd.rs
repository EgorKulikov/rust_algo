//{"name":"D. Cow the Tree Nerd","group":"Codeforces - TheForces Round #38 (Tree-Forces)","url":"https://codeforces.com/gym/105622/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 2 3\n2 3 -1\n3 4 2\n4 5 -1\n4 6 -130\n","output":"4\n"},{"input":"5\n1 2 8\n2 3 -1\n3 4 -100\n4 5 10\n","output":"17\n"},{"input":"2\n1 2 -10\n","output":"-10\n"},{"input":"3\n1 2 100\n1 3 150\n","output":"150\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCowTheTreeNerd"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(n - 1).dec();

    let mut graph = Graph::new(n);
    for (u, v, _) in edges.copy_iter() {
        graph.add_edge(BiEdge::new(u, v));
    }
    let (_, _, w) = edges.detuple();
    let mut pos_w = w.copy_filter(|&x| x > 0).collect_vec();
    let mut neg_w = w.copy_filter(|&x| x < 0).collect_vec();
    pos_w.sort_unstable();
    pos_w.reverse();
    neg_w.sort_unstable();
    neg_w.reverse();
    let dist = graph.edge_distances(0);
    let end = dist.into_iter().max_position().unwrap();
    let dist = graph.edge_distances(end);
    let max_len = dist.iter_max() as usize;
    let mut ans = if pos_w.is_empty() { neg_w[0] } else { pos_w[0] };
    let mut len = 1;
    while 2 * len + 1 <= max_len
        && pos_w.len() > len
        && neg_w.len() >= len
        && neg_w[len - 1] + pos_w[len] > 0
    {
        ans += neg_w[len - 1] + pos_w[len];
        len += 1;
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}

#[cfg(test)]
mod tester;
//END MAIN
