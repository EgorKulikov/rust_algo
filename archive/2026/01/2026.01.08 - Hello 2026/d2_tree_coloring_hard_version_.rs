//{"name":"D2. Tree Coloring (Hard Version)","group":"Codeforces - Hello 2026","url":"https://codeforces.com/contest/2183/problem/D2","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n5\n3 1\n1 2\n5 1\n4 1\n5\n3 2\n2 4\n2 5\n1 2\n5\n3 4\n4 1\n5 1\n1 2\n5\n2 5\n3 1\n2 1\n3 4\n5\n1 3\n1 5\n4 3\n2 4\n13\n2 1\n3 2\n4 2\n5 4\n6 3\n7 1\n8 5\n9 6\n10 4\n11 7\n12 8\n13 10\n10\n5 7\n8 1\n1 10\n2 8\n8 4\n9 4\n6 1\n5 3\n7 8\n10\n7 6\n3 7\n6 9\n7 1\n9 8\n5 1\n3 10\n9 2\n1 4\n10\n10 6\n2 8\n4 10\n7 5\n1 2\n7 10\n10 9\n9 1\n7 3\n10\n6 8\n9 7\n4 10\n5 9\n4 2\n3 8\n6 5\n1 5\n1 10\n","output":"5\n1 3\n1 2\n1 5\n1 4\n1 1\n4\n2 3 1\n1 4\n1 5\n1 2\n4\n1 4\n2 5 3\n1 2\n1 1\n3\n2 4 2\n2 5 3\n1 1\n3\n2 3 2\n2 5 4\n1 1\n3\n5 9 12 10 11 2\n4 8 6 4 1\n4 13 5 3 7\n4\n4 2 9 3 10\n3 4 5 6\n2 7 1\n1 8\n4\n2 7 9\n3 5 3 2\n4 4 6 10 8\n1 1\n4\n4 6 3 8 9\n3 4 5 1\n1 7\n2 10 2\n3\n4 7 3 4 5\n3 8 9 10\n3 2 6 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let d = graph.edge_distances(0);
    let mut pos = vec![Vec::new(); n];
    for i in 0..n {
        pos[d[i] as usize].push(i);
    }
    let mut col = vec![n; n];
    col[0] = 0;
    let lca = graph.lca();
    for i in 1..n {
        let mut free = None;
        let mut cur = 0;
        for v in pos[i].copy_iter() {
            if let Some(f) = free {
                if col[lca.parent(v).unwrap()] != f {
                    col[v] = f;
                    free = None;
                    continue;
                }
            }
            if col[lca.parent(v).unwrap()] != cur {
                col[v] = cur;
            } else {
                free = Some(cur);
                cur += 1;
                col[v] = cur;
            }
            cur += 1;
        }
        if cur != pos[i].len() {
            let last = pos[i][Back(0)];
            for j in 0..pos[i].len() - 1 {
                if col[pos[i][j]] != col[lca.parent(last).unwrap()]
                    && free.unwrap() != col[lca.parent(pos[i][j]).unwrap()]
                {
                    col[last] = col[pos[i][j]];
                    col[pos[i][j]] = free.unwrap();
                    break;
                }
            }
        }
    }
    for (u, v) in edges.copy_iter() {
        assert_ne!(col[u], col[v]);
    }
    for i in 0..n {
        let mut set = FxHashSet::default();
        for v in pos[i].copy_iter() {
            set.insert(col[v]);
        }
        assert_eq!(set.len(), pos[i].len());
    }
    let b = by_index(&col);
    out.print_line(b.len());
    for v in b.into_values() {
        out.print_line((v.len(), v.inc()));
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
