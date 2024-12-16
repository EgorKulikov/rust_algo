//{"name":"day_16","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_16"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(input.read_line());
    }

    // part 1
    {
        let n = data.len();
        let m = data[0].len();
        let mut graph = Graph::new(n * m * 4);
        let mut start = 0;
        let mut end = 0;
        for i in 0..n {
            for j in 0..m {
                if data[i][j] == b'#' {
                    continue;
                }
                let mut dr = 1;
                let mut dc = 0;
                for k in 0..4 {
                    let next = (k + 1) % 4;
                    graph.add_edge(WeightedEdge::new(
                        i * m * 4 + j * 4 + k,
                        i * m * 4 + j * 4 + next,
                        1000,
                    ));
                    let prev = (k + 3) % 4;
                    graph.add_edge(WeightedEdge::new(
                        i * m * 4 + j * 4 + k,
                        i * m * 4 + j * 4 + prev,
                        1000,
                    ));
                    let ni = i as i32 + dr;
                    let nj = j as i32 + dc;
                    if ni >= 0
                        && ni < n as i32
                        && nj >= 0
                        && nj < m as i32
                        && data[ni as usize][nj as usize] != b'#'
                    {
                        graph.add_edge(WeightedEdge::new(
                            i * m * 4 + j * 4 + k,
                            ni as usize * m * 4 + nj as usize * 4 + k,
                            1,
                        ));
                    }
                    (dr, dc) = (dc, -dr);
                }
                if data[i][j] == b'S' {
                    start = i * m * 4 + j * 4;
                }
                if data[i][j] == b'E' {
                    end = i * m * 4 + j * 4;
                }
            }
        }
        let mut ans = None;
        // for s in start..start + 4 {
        let dist = graph.distances_from(start + 3);
        for e in end..end + 4 {
            if let Some((d, ..)) = dist[e] {
                ans.minim(d);
            }
        }
        // }
        out.print_line(ans);
    }

    // part 2
    {
        let n = data.len();
        let m = data[0].len();
        let mut graph = Graph::new(n * m * 4);
        let mut graph_t = Graph::new(n * m * 4);
        let mut start = 0;
        let mut end = 0;
        for i in 0..n {
            for j in 0..m {
                if data[i][j] == b'#' {
                    continue;
                }
                let mut dr = 1;
                let mut dc = 0;
                for k in 0..4 {
                    let next = (k + 1) % 4;
                    graph.add_edge(WeightedEdge::new(
                        i * m * 4 + j * 4 + k,
                        i * m * 4 + j * 4 + next,
                        1000,
                    ));
                    graph_t.add_edge(WeightedEdge::new(
                        i * m * 4 + j * 4 + next,
                        i * m * 4 + j * 4 + k,
                        1000,
                    ));
                    let prev = (k + 3) % 4;
                    graph.add_edge(WeightedEdge::new(
                        i * m * 4 + j * 4 + k,
                        i * m * 4 + j * 4 + prev,
                        1000,
                    ));
                    graph_t.add_edge(WeightedEdge::new(
                        i * m * 4 + j * 4 + prev,
                        i * m * 4 + j * 4 + k,
                        1000,
                    ));
                    let ni = i as i32 + dr;
                    let nj = j as i32 + dc;
                    if ni >= 0
                        && ni < n as i32
                        && nj >= 0
                        && nj < m as i32
                        && data[ni as usize][nj as usize] != b'#'
                    {
                        graph.add_edge(WeightedEdge::new(
                            i * m * 4 + j * 4 + k,
                            ni as usize * m * 4 + nj as usize * 4 + k,
                            1,
                        ));
                        graph_t.add_edge(WeightedEdge::new(
                            ni as usize * m * 4 + nj as usize * 4 + k,
                            i * m * 4 + j * 4 + k,
                            1,
                        ));
                    }
                    (dr, dc) = (dc, -dr);
                }
                if data[i][j] == b'S' {
                    start = i * m * 4 + j * 4;
                }
                if data[i][j] == b'E' {
                    end = i * m * 4 + j * 4;
                }
            }
        }
        let d_start = graph.distances_from(start + 3);
        let mut ans = None;
        for e in end..end + 4 {
            if let Some((d, ..)) = d_start[e] {
                ans.minim(d);
            }
        }
        let ans = ans.unwrap();
        let d_end = Vec::gen(4, |i, _| graph_t.distances_from(end + i));
        let mut qty = 0;
        for id in 0..n * m {
            let mut found = false;
            for i in 0..4 {
                if let Some((d0, ..)) = d_start[id * 4 + i] {
                    for j in 0..4 {
                        if let Some((d1, ..)) = d_end[j][id * 4 + i] {
                            if d0 + d1 == ans {
                                found = true;
                                break;
                            }
                        }
                    }
                }
            }
            if found {
                qty += 1;
            }
        }
        out.print_line(qty);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
