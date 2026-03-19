//{"name":"And again trees","group":"Eolymp - Basecamp - Weekend Practice #20","url":"https://eolymp.com/en/compete/4pe1cne4r571j089budf65juq4/problem/5","interactive":false,"timeLimit":1000,"tests":[{"input":"dfs(v):\n    write the number in vertex v to the end of array a\n    choose any order of the children of vertex v\n    for each child u in the chosen order:\n        dfs(u)\n","output":"7\n5 2 8 6 3 9 7\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::next_permutation::NextPermutation;
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
    let a = input.read_int_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    if graph[0].len() == n - 1 {
        let mut more = FxHashSet::default();
        for i in 1..n {
            if a[i] > a[0] {
                more.insert(a[i]);
            }
        }
        out.print_line(more.len() + 1);
        return;
    }
    if (1..n - 1).all(|i| graph[i].len() == 2) {
        let mut ans = 1;
        let mut cur = a[0];
        for i in 1..n {
            if cur.maxim(a[i]) {
                ans += 1;
            }
        }
        out.print_line(ans);
        return;
    }
    if n <= 10 {
        let mut order = (0..n).collect::<Vec<_>>();
        let mut ans = 0;
        loop {
            let mut cur = 0;
            let mut res = 0;
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                if cur.maxim(a[vert]) {
                    res += 1;
                }
                let mut t = Vec::new();
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    t.push(e.to());
                    // f.call(e.to(), vert);
                }
                t.sort_by_key(|&x| a[x]);
                for u in t {
                    f.call(u, vert);
                }
            });
            dfs.call(0, 0);
            ans.maxim(res);
            if !order.next_permutation() {
                break;
            }
        }
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
