//{"name":"Markers on a Tree (Easy)","group":"CodeChef - START215A","url":"https://www.codechef.com/START215A/problems/MARKTREE","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1\n01\n3\n1 2\n111\n5\n1 2 1 3\n01011\n7\n1 2 1 4 4 2\n0011010\n6\n1 2 3 4 5\n110011\n","output":"0\n2\n4\n8\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n - 1).dec();
    let mut x = input.read_str();

    x[0] = b'1';
    x[n - 1] = b'1';
    let mut val = DefaultHashMap::new(0);
    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        graph.add_edge(BiEdge::new(p[i], i + 1));
    }
    let mut ans = 0;
    for r in [0, n - 1] {
        let mut q = vec![0; n];
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
            q[vert] = if x[vert] == b'1' { 1 } else { 0 };
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert);
                q[vert] += q[e.to()];
            }
        });
        dfs.call(r, n);
        let mut dfs =
            RecursiveFunction3::new(|f, vert: usize, prev: usize, single: i64| -> (i64, bool) {
                let mut val_here = 0;
                let (cur_single, delta) = if x[vert] == b'1' { (0, 1) } else { (single, 0) };
                let mut on_path = vert == 0 || vert == n - 1;
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    let c_single = if q[e.to()] + delta == q[vert] {
                        cur_single + 1
                    } else {
                        1
                    };
                    if q[e.to()] != 0 {
                        let (call_val, call_path) = f.call(e.to(), vert, c_single);
                        on_path |= call_path;
                        val_here += 2 + call_val;
                    }
                }
                if prev != n && on_path {
                    val[(vert.min(prev), vert.max(prev))] += val_here + 2 * single;
                }
                (val_here, on_path)
            });
        ans += dfs.call(r, n, 0).0;
    }
    out.print_line(ans - val.into_values().max().unwrap_or(0));
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
