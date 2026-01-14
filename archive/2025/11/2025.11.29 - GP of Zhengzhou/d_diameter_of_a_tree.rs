//{"name":"D. Diameter of a Tree","group":"Universal Cup - GP of Zhengzhou","url":"https://contest.ucup.ac/contest/2661/problem/15304","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 2\n1 3\n2 4\n2 5\n4\n1 2\n1 3\n1 4\n6\n1 2\n1 3\n2 4\n2 5\n3 6\n8\n1 2\n2 3\n1 4\n2 5\n3 6\n1 7\n7 8\n10\n1 2\n2 3\n2 4\n4 7\n4 8\n1 5\n5 6\n6 9\n1 10\n","output":"3 4 5 1\n3 4 2\n3 4 5 6 1\n2 3 4 5 6 1\n3 4 5 6 7 8 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::BiEdgeAlgos;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    if n == 2 {
        out.print_line((2, 1));
        return;
    }
    let graph = Graph::with_biedges(n, &edges);
    let centers = graph.centers();
    let diameter = graph.diameter();
    let mut path = None;
    let mut ends = 0;
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, len: usize| {
        let mut cur = if len == diameter / 2 {
            ends += 1;
            Some(VecDeque::new())
        } else {
            None
        };
        let mut multi = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            if let Some(v) = f.call(e.to(), vert, len + 1) {
                multi += 1;
                cur.minim(v);
            }
        }
        if let Some(mut v) = cur {
            v.push_front(multi);
            Some(v)
        } else {
            None
        }
    });
    for i in centers.indices() {
        path.minim(dfs.call(centers[i], centers[Back(i)], 0).unwrap());
    }
    let mut path = path.unwrap();
    if centers.len() == 1 {
        path[0] -= 1;
    }
    let mut ans = Vec::with_capacity(diameter + 1);
    let mut next = ends;
    for _ in 0..diameter.upper_div(2) {
        ans.push(next);
        next += 1;
    }
    for i in path.copy_iter() {
        ans.push(next);
        next += i;
    }
    ans.pop();
    ans.push(path[path.len() - 2]);
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
