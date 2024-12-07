//{"name":"G. Tree Destruction","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2\n1 2\n5\n1 2\n2 3\n3 4\n3 5\n4\n1 2\n2 3\n3 4\n5\n2 1\n3 1\n4 1\n5 4\n6\n2 1\n3 1\n4 1\n5 3\n6 3\n6\n2 1\n3 2\n4 2\n5 3\n6 4\n","output":"1\n3\n2\n3\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GTreeDestruction"}}}

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

    let graph = Graph::from_biedges(n, &edges);
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
        let mut best = None;
        let mut children = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            if let Some(best) = best {
                ans.maxim(best + call + graph[vert].len() - 2);
            }
            ans.maxim(call + graph[vert].len() - 1);
            best.maxim(call);
            children += 1;
        }
        ans.maxim(graph[vert].len());
        best = best.map(|x| x + children - 1);
        best.maxim(children);
        best.unwrap()
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
