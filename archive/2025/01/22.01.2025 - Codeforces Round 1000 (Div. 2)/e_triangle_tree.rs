//{"name":"E. Triangle Tree","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 2\n1 3\n3\n1 2\n3 2\n5\n2 3\n1 5\n4 2\n1 2\n11\n2 1\n2 3\n2 4\n4 5\n6 5\n5 7\n4 8\n8 9\n7 10\n10 11\n","output":"1\n0\n4\n29\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use std::collections::VecDeque;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> VecDeque<usize> {
        let mut res = VecDeque::new();
        res.push_back(0);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let mut call = f.call(e.to(), vert);
            if call.len() > res.len() {
                swap(&mut call, &mut res);
            }
            for i in 0..call.len() - 1 {
                let cr = res[i] - res[i + 1];
                let cc = call[i] - call[i + 1];
                ans += (2 * i + 1) * (cr * call[i + 1] + cc * res[i]);
                res[i] += call[i];
            }
        }
        res.push_front(1 + res[0]);
        res
    });
    dfs.call(0, n);
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

//START MAIN
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
//END MAIN
