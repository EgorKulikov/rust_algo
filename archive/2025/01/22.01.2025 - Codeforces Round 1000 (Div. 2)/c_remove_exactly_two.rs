//{"name":"C. Remove Exactly Two","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n4\n1 2\n2 3\n2 4\n7\n1 2\n1 3\n2 4\n4 5\n5 6\n5 7\n","output":"0\n2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (usize, usize) {
        let mut children = 0;
        let mut res = 0;
        let mut max_res = 0;
        let mut second_max_res = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (call_res, call_children) = f.call(e.to(), vert);
            let cand = call_res.max(call_children + 1);
            res.maxim(call_res);
            if cand > max_res {
                second_max_res = max_res;
                max_res = cand;
            } else if cand > second_max_res {
                second_max_res = cand;
            }
            children += 1;
        }
        if children > 0 {
            ans.maxim(children - 1 + res + if vert == 0 { 0 } else { 1 });
        }
        if second_max_res > 0 {
            ans.maxim(max_res + second_max_res - 1);
        }
        max_res.maxim(children);
        (max_res, children)
    });
    dfs.call(0, 0);
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
