//{"name":"E. Mirror I","group":"Codeforces - TheForces Round #38 (Tree-Forces)","url":"https://codeforces.com/gym/105622/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n1 2 2 4\n","output":"4\n"},{"input":"5 5\n1 2 3 4\n","output":"0\n"},{"input":"4 4\n1 1 1\n","output":"0\n"},{"input":"4 1\n1 1 1\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMirrorI"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let p = input.read_size_vec(n - 1).dec();

    let mut graph = Graph::new(n);
    for i in 1..n {
        graph.add_edge(Edge::new(p[i - 1], i));
    }
    let mut ans = 0usize;
    let mut rec = RecursiveFunction2::new(|rec, vert: usize, depth: usize| -> VecDeque<usize> {
        let mut res = VecDeque::from(vec![1usize]);
        for e in &graph[vert] {
            let mut call = rec.call(e.to(), depth + 1);
            call.push_front(0);
            if call.len() > k {
                call[0] += call[k];
                call.pop_back();
            }
            if call.len() > res.len() {
                swap(&mut res, &mut call);
            }
            for i in 0..call.len() {
                if i + depth >= k || i == 0 {
                    ans += call[i] * res[i];
                }
                res[i] += call[i];
            }
        }
        res
    });
    rec.call(0, 0);
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
//END MAIN
