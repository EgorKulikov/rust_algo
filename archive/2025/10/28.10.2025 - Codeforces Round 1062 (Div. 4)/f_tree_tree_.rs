//{"name":"F. Tree, TREE!!!","group":"Codeforces - Codeforces Round 1062 (Div. 4)","url":"https://codeforces.com/contest/2167/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 2\n1 2\n5 3\n1 2\n1 3\n1 4\n1 5\n6 3\n1 2\n1 3\n2 4\n2 5\n3 6\n10 5\n5 6\n4 9\n3 9\n2 6\n2 8\n8 9\n6 10\n1 6\n4 7\n","output":"2\n9\n17\n35\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let k = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut ans = n;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        let mut sz = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            sz += f.call(e.to(), vert);
        }
        if sz >= k {
            ans += n - sz;
        }
        if n - sz >= k {
            ans += sz;
        }
        sz
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
