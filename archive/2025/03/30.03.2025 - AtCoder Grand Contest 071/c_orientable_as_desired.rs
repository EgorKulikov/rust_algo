//{"name":"C - Orientable as Desired","group":"AtCoder - AtCoder Grand Contest 071","url":"https://atcoder.jp/contests/agc071/tasks/agc071_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 4\n1 2\n2 3\n3 4\n4 1\n7 6\n3 4\n3 6\n2 5\n1 2\n1 3\n2 7\n15 20\n11 13\n11 7\n12 6\n5 2\n4 1\n15 3\n9 8\n13 5\n8 6\n7 5\n11 8\n12 9\n14 1\n1 11\n6 7\n1 3\n2 3\n3 7\n5 12\n10 6\n","output":"Yes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let mut graph = Graph::new(n);
    for (u, v) in edges.copy_iter() {
        graph.add_edge(BiEdgeWithId::new(u, v));
    }
    let mut col = vec![0; n];
    let mut two_color = true;
    let mut dfs = RecursiveFunction2::new(|rec, vert: usize, c: i32| {
        if col[vert] != 0 {
            if col[vert] != c {
                two_color = false;
            }
            return;
        }
        col[vert] = c;
        for e in &graph[vert] {
            rec.call(e.to(), -c);
        }
    });
    dfs.call(0, 1);
    if !two_color {
        out.print_line(true);
        return;
    }
    let mut timer = 0;
    let mut tin = vec![0; n];
    let mut fup = vec![0; n];
    let mut used = BitSet::new(n);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        used.set(vert);
        tin[vert] = timer;
        fup[vert] = timer;
        timer += 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let to = e.to();
            if used[to] {
                fup[vert].minim(tin[to]);
            } else {
                f.call(to, vert);
                let cand = fup[to];
                fup[vert].minim(cand);
            }
        }
    });
    dfs.call(0, n);
    let mut max_color = 0;
    let mut color = vec![n; m];
    let mut visited = BitSet::new(n);
    let mut paint = RecursiveFunction3::new(|rec, vert: usize, prev: usize, col: usize| {
        visited.set(vert);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let u = e.to();
            if !visited[u] {
                if fup[u] >= tin[vert] {
                    max_color += 1;
                    let new_color = max_color;
                    color[e.id()] = new_color;
                    rec.call(u, vert, new_color);
                } else {
                    color[e.id()] = col;
                    rec.call(u, vert, col);
                }
            } else if tin[u] < tin[vert] {
                color[e.id()] = col;
            }
        }
    });
    paint.call(0, n, 0);
    for i in 0..n {
        let mut by_color = DefaultHashMap::new(0);
        for e in &graph[i] {
            by_color[color[e.id()]] += 1;
        }
        let q = by_color.into_values().collect::<Vec<_>>().sorted();
        let mut sum = 0;
        for i in q {
            if sum + 1 >= i {
                sum += i;
            } else {
                out.print_line(true);
                return;
            }
        }
    }
    out.print_line(false);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
