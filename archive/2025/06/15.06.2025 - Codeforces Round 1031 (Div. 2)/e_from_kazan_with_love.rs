//{"name":"E. From Kazan with Love","group":"Codeforces - Codeforces Round 1031 (Div. 2)","url":"https://codeforces.com/contest/2113/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n4 1 1 4\n1 2\n2 3\n3 4\n4 1\n5 1 1 5\n1 2\n2 3\n3 4\n4 5\n5 1\n9 2 1 9\n1 2\n2 3\n3 4\n3 5\n5 6\n6 7\n6 8\n8 9\n9 1\n7 1\n9 2 7 2\n1 4\n2 5\n3 6\n4 5\n5 6\n4 7\n5 8\n6 9\n2 8\n3 7\n","output":"4\n6\n10\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let x = input.read_size() - 1;
    let y = input.read_size() - 1;
    let edges = input.read_size_pair_vec(n - 1).dec();
    let ab = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_biedges(n, &edges);
    let lca = graph.lca();
    let mut forbidden = vec![Vec::new(); n];
    for (a, b) in ab {
        let len = lca.path_length(a, b);
        for j in 0..=len {
            forbidden[lca.nth_vert_on_path(a, b, j)].push(j);
        }
    }
    for i in 0..n {
        forbidden[i].sort_unstable();
        forbidden[i].dedup();
    }

    let mut start = vec![Vec::default(); 2 * n + 1];
    let mut end = vec![Vec::default(); 2 * n + 2];
    for i in 0..n {
        let c_start = lca.path_length(x, i);
        let c_end = 2 * n;
        if c_start <= c_end {
            let mut last_start = c_start;
            for &j in &forbidden[i] {
                if j < c_start || j > c_end {
                    continue;
                }
                if j == last_start {
                    last_start += 1;
                } else {
                    end[j].push(i);
                    start[last_start].push(i);
                    last_start = j + 1;
                }
            }
            if last_start != c_end + 1 {
                end[c_end + 1].push(i);
                start[last_start].push(i);
            }
        }
    }
    let mut cur = FxHashSet::default();
    for &i in &start[0] {
        cur.insert(i);
    }
    let mut eligible = FxHashSet::default();
    let mut last_added = cur.clone();
    for i in 1..2 * n {
        if cur.contains(&y) {
            out.print_line(i);
            return;
        }
        for j in &start[i] {
            eligible.insert(*j);
        }
        for j in &end[i] {
            eligible.remove(j);
        }
        let mut to_add = FxHashSet::default();
        for j in &last_added {
            for e in &graph[*j] {
                if eligible.contains(&e.to()) && !cur.contains(&e.to()) {
                    to_add.insert(e.to());
                }
            }
        }
        for j in &start[i] {
            for e in &graph[*j] {
                if cur.contains(&e.to()) {
                    to_add.insert(*j);
                    break;
                }
            }
        }
        for j in &end[i] {
            cur.remove(j);
        }
        for j in &to_add {
            cur.insert(*j);
        }
        last_added = to_add;
        if cur.is_empty() {
            out.print_line(-1);
            return;
        }
    }
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
