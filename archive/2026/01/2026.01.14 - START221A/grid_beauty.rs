//{"name":"Grid Beauty","group":"CodeChef - START221A","url":"https://www.codechef.com/START221A/problems/LMP6","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 1\n1\n2 2\n1 2\n3 4\n2 2\n1 3\n2 4\n1 5\n1 3 2 5 4\n3 3\n4 1 3\n9 2 6\n5 7 8\n","output":"2\n5\n4\n3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu2d::DSU2d;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::time_tracker::TimeTracker;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut tt = TimeTracker::new();
    let n = input.read_size();
    let m = input.read_size();
    let g = input.read_size_table(n, m);

    let mut done = Arr2d::new(n, m, false);
    let mut cycles = Vec::new();
    for (i, j) in g.indices() {
        if done[(i, j)] {
            continue;
        }
        let mut cycle = Vec::new();
        let mut cur = (i, j);
        while !done[cur] {
            cycle.push(cur);
            done[cur] = true;
            cur = ((g[cur] - 1) / m, (g[cur] - 1) % m);
        }
        cycles.push(cycle);
    }
    tt.milestone("prep");
    let mut left = 1;
    let mut right = n * m + 1;
    let mut dsu = DSU2d::new(n, m);
    while left < right {
        let mid = (left + right + 1) / 2;
        // let mut graph = Graph::new(5 * n * m);
        // if n < m {
        //     for i in 0..n {
        //         for j in 0..=i {
        //             let x = mid.upper_div(i - j + 1);
        //             for k in x - 1..m {
        //                 graph.add_edge(Edge::new(i * m + k, 2 * n * m + j * m + k - (x - 1)));
        //             }
        //             for k in 0..m.saturating_sub(x - 1) {
        //                 graph.add_edge(Edge::new(i * m + k, n * m + j * m + k + x - 1));
        //             }
        //         }
        //         for j in i..n {
        //             let x = mid.upper_div(j - i + 1);
        //             for k in 0..m.saturating_sub(x - 1) {
        //                 graph.add_edge(Edge::new(i * m + k, 3 * n * m + j * m + k + x - 1));
        //             }
        //             for k in x - 1..m {
        //                 graph.add_edge(Edge::new(i * m + k, 4 * n * m + j * m + k - (x - 1)));
        //             }
        //         }
        //     }
        // } else {
        //     for i in 0..m {
        //         for j in 0..=i {
        //             let x = mid.upper_div(i - j + 1);
        //             for k in 0..n.saturating_sub(x - 1) {
        //                 graph.add_edge(Edge::new(k * m + i, 4 * n * m + (k + x - 1) * m + j));
        //             }
        //             for k in x - 1..n {
        //                 graph.add_edge(Edge::new(k * m + i, 2 * n * m + (k - (x - 1)) * m + j));
        //             }
        //         }
        //         for j in i..m {
        //             let x = mid.upper_div(j - i + 1);
        //             for k in 0..n.saturating_sub(x - 1) {
        //                 graph.add_edge(Edge::new(k * m + i, 3 * n * m + (k + x - 1) * m + j));
        //             }
        //             for k in x - 1..n {
        //                 graph.add_edge(Edge::new(k * m + i, n * m + (k - (x - 1)) * m + j));
        //             }
        //         }
        //     }
        // }
        // for i in 0..n {
        //     for j in 0..m {
        //         graph.add_edge(Edge::new(n * m + i * m + j, i * m + j));
        //         graph.add_edge(Edge::new(2 * n * m + i * m + j, i * m + j));
        //         graph.add_edge(Edge::new(3 * n * m + i * m + j, i * m + j));
        //         graph.add_edge(Edge::new(4 * n * m + i * m + j, i * m + j));
        //         if j + 1 < m {
        //             graph.add_edge(Edge::new(n * m + i * m + j, n * m + i * m + (j + 1)));
        //             graph.add_edge(Edge::new(
        //                 3 * n * m + i * m + j,
        //                 3 * n * m + i * m + (j + 1),
        //             ));
        //         }
        //         if i + 1 < n {
        //             graph.add_edge(Edge::new(
        //                 3 * n * m + i * m + j,
        //                 3 * n * m + (i + 1) * m + j,
        //             ));
        //             graph.add_edge(Edge::new(
        //                 4 * n * m + i * m + j,
        //                 4 * n * m + (i + 1) * m + j,
        //             ));
        //         }
        //         if i > 0 {
        //             graph.add_edge(Edge::new(
        //                 2 * n * m + i * m + j,
        //                 2 * n * m + (i - 1) * m + j,
        //             ));
        //             graph.add_edge(Edge::new(n * m + i * m + j, n * m + (i - 1) * m + j));
        //         }
        //         if j > 0 {
        //             graph.add_edge(Edge::new(
        //                 4 * n * m + i * m + j,
        //                 4 * n * m + i * m + (j - 1),
        //             ));
        //             graph.add_edge(Edge::new(
        //                 2 * n * m + i * m + j,
        //                 2 * n * m + i * m + (j - 1),
        //             ));
        //         }
        //     }
        // }
        // let col = graph.strongly_connected_component_colors();
        dsu.clear();
        for (i, j) in g.indices() {
            if (i + 1) * (j + 1) >= mid {
                dsu.union(i, j, 0, 0);
            }
            if (i + 1) * (m - j) >= mid {
                dsu.union(i, j, 0, m - 1);
            }
            if (n - i) * (j + 1) >= mid {
                dsu.union(i, j, n - 1, 0);
            }
            if (n - i) * (m - j) >= mid {
                dsu.union(i, j, n - 1, m - 1);
            }
        }
        let mut ok = true;
        for cycle in &cycles {
            let base = dsu.find(cycle[0].0, cycle[0].1);
            for &(x, y) in cycle.iter() {
                if dsu.find(x, y) != base {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if ok {
            left = mid;
        } else {
            right = mid - 1;
        }
        tt.milestone(format!("mid {}", mid).as_str());
    }
    out.print_line(left);
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
