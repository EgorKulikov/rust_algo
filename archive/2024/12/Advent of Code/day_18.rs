//{"name":"day_18","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_18"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::{output, scan};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p1 = input.read_size();
    let mut data = Vec::new();
    while !input.is_empty() {
        scan!(input, "@,@", x: usize, y: usize);
        data.push((x, y));
    }

    // part 1
    {
        let mut map = Arr2d::new(n, n, false);
        for (x, y) in data.copy_take(p1) {
            map[(x, y)] = true;
        }
        let mut graph = Graph::new(n * n);
        for i in 0..n {
            for j in 0..n {
                if map[(i, j)] {
                    continue;
                }
                if i + 1 < n && !map[(i + 1, j)] {
                    graph.add_edge(BiEdge::new(i * n + j, (i + 1) * n + j));
                }
                if j + 1 < n && !map[(i, j + 1)] {
                    graph.add_edge(BiEdge::new(i * n + j, i * n + j + 1));
                }
            }
        }
        out.print_line(graph.edge_distances(0)[Back(0)]);
    }

    // part 2
    {
        for i in p1 + 1..data.len() {
            let mut map = Arr2d::new(n, n, false);
            for (x, y) in data.copy_take(i) {
                map[(x, y)] = true;
            }
            let mut graph = Graph::new(n * n);
            for i in 0..n {
                for j in 0..n {
                    if map[(i, j)] {
                        continue;
                    }
                    if i + 1 < n && !map[(i + 1, j)] {
                        graph.add_edge(BiEdge::new(i * n + j, (i + 1) * n + j));
                    }
                    if j + 1 < n && !map[(i, j + 1)] {
                        graph.add_edge(BiEdge::new(i * n + j, i * n + j + 1));
                    }
                }
            }
            if graph.edge_distances(0)[Back(0)] == u32::MAX {
                output!(out, "{},{}", data[i - 1].0, data[i - 1].1);
                return;
            }
        }
    }
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}

#[cfg(test)]
mod tester;
//END MAIN
