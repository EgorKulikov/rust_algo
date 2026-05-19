//{"name":"F. Игра на растущем дереве","group":"Codeforces - Educational Codeforces Round 190 (Rated for Div. 2)","url":"https://codeforces.com/contest/2230/problem/F","interactive":false,"timeLimit":6000,"tests":[{"input":"9\n1 1 3 3 1 2 1 2 8\n","output":"1 2 2 2 2 2 2 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let q = input.read_size();
    let v = input.read_size_vec(q).dec();

    let mut graph = Graph::new(q + 1, q);
    for i in 0..q {
        graph.add_edge(BiEdge::new(v[i], i + 1));
    }
    let mut ans = vec![1; q];
    let mut edge = vec![Vec::new(); q + 1];
    for i in 0..=q {
        for e in graph.adj(i) {
            edge[i].push((e.to(), e.to()));
        }
        edge[i].sort();
    }
    let mut next_edge = vec![Vec::new(); q + 1];
    for x in 2.. {
        let mut found = false;
        for i in 0..=q {
            if edge[i].len() >= 2 {
                ans[edge[i][1].0 - 1] = x;
                found = true;
            }
        }
        if !found {
            break;
        }
        for i in 0..=q {
            next_edge[i].clear();
            for e in graph.adj(i) {
                let to = e.to();
                let mut found = 0;
                let mut min = 0;
                for j in edge[to].indices() {
                    if edge[to][j].1 != i {
                        found += 1;
                        min.maxim(edge[to][j].0);
                        if found == 2 {
                            break;
                        }
                    }
                }
                if found == 2 {
                    next_edge[i].push((min, to));
                }
            }
            next_edge[i].sort();
        }
        swap(&mut edge, &mut next_edge);
    }
    for i in 1..q {
        let cur = ans[i - 1];
        ans[i].maxim(cur);
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
