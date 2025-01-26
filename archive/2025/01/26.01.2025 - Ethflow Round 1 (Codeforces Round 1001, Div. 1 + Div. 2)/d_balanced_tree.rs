//{"name":"D. Balanced Tree","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n4\n0 11\n6 6\n0 0\n5 5\n2 1\n3 1\n4 3\n7\n1 1\n0 5\n0 5\n2 2\n2 2\n2 2\n2 2\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n4\n1 1\n1 1\n1 1\n0 0\n1 4\n2 4\n3 4\n7\n0 20\n0 20\n0 20\n0 20\n3 3\n4 4\n5 5\n1 2\n1 3\n1 4\n2 5\n3 6\n4 7\n5\n1000000000 1000000000\n0 0\n1000000000 1000000000\n0 0\n1000000000 1000000000\n3 2\n2 1\n1 4\n4 5\n6\n21 88\n57 81\n98 99\n61 76\n15 50\n23 67\n2 1\n3 2\n4 3\n5 3\n6 4\n","output":"11\n3\n3\n5\n3000000000\n98\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let lr = input.read_long_pair_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);

    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (i64, i64) {
        let mut add = 0;
        let mut vals = Vec::new();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (call_val, call_add) = f.call(e.to(), vert);
            add += call_add;
            vals.push(call_val - call_add);
        }
        let mn = lr[vert].0 + add;
        let mx = lr[vert].1 + add;
        let mut res = mn;
        let was_add = add;
        let mut add_add = 0;
        let mut max_add_add = 0;
        for mut v in vals {
            v += was_add;
            if v <= mx {
                res.maxim(v);
            } else {
                add += v - mx;
                add_add += v - mx;
                res.maxim(v);
                max_add_add.maxim(v - mx);
            }
        }
        (res + add_add - max_add_add, add)
    });
    out.print_line(dfs.call(0, n).0);
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
