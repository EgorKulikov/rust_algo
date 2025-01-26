//{"name":"D - Moving Pieces on Graph","group":"AtCoder - AtCoder Regular Contest 191 (Div. 2)","url":"https://atcoder.jp/contests/arc191/tasks/arc191_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 3 4\n2 4\n1 4\n3 4\n2 3\n","output":"3\n"},{"input":"2 1 1 2\n1 2\n","output":"-1\n"},{"input":"5 6 3 5\n1 2\n2 3\n1 5\n2 4\n1 3\n2 5\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_size() - 1;
    let t = input.read_size() - 1;
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_biedges(n, &edges);
    let ds = graph.edge_distances(s);
    let dt = graph.edge_distances(t);

    let direct = ds[t];
    let mut by_d = DefaultHashMap::new(Vec::new());
    for i in 0..n {
        by_d[(ds[i], dt[i])].push(i);
    }
    let mut ans = None;
    for ((ds, dt), verts) in by_d {
        if verts.len() > 1 {
            ans.minim(2 * (ds + dt));
        }
        if ds + dt != direct && ds.abs_diff(dt) != direct {
            ans.minim(direct + ds + dt);
        }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
