//{"name":"day_20","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_20"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let min_save = input.read_unsigned();
    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(input.read_line());
    }

    let mut sr = 0;
    let mut sc = 0;
    let mut er = 0;
    let mut ec = 0;
    let n = data.len();
    let m = data[0].len();
    let mut graph = Graph::new(n * m);
    for i in 0..n {
        for j in 0..m {
            if data[i][j] == b'S' {
                sr = i;
                sc = j;
            } else if data[i][j] == b'E' {
                er = i;
                ec = j;
            }
            if data[i][j] == b'#' {
                continue;
            }
            if i + 1 < n && data[i + 1][j] != b'#' {
                graph.add_edge(BiEdge::new(i * m + j, (i + 1) * m + j));
            }
            if j + 1 < m && data[i][j + 1] != b'#' {
                graph.add_edge(BiEdge::new(i * m + j, i * m + j + 1));
            }
        }
    }
    let from_start = graph.edge_distances(sr * m + sc);
    let from_end = graph.edge_distances(er * m + ec);
    let base = from_start[er * m + ec];

    // part 1
    {
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if data[i][j] != b'#' {
                    continue;
                }
                let mut to_start = (n * m) as u32;
                let mut to_end = (n * m) as u32;
                for (r, c) in D4::iter(i, j, n, m) {
                    to_start.minim(from_start[r * m + c]);
                    to_end.minim(from_end[r * m + c]);
                }
                let cand = to_start + to_end + 2;
                if cand + min_save <= base {
                    ans += 1;
                }
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if data[i][j] == b'#' {
                    continue;
                }
                for r in 0..n {
                    for c in 0..m {
                        if data[r][c] == b'#' {
                            continue;
                        }
                        if i.abs_diff(r) + j.abs_diff(c) > 20 {
                            continue;
                        }
                        let to_start = from_start[i * m + j];
                        let to_end = from_end[r * m + c];
                        let cand = to_start + to_end + i.abs_diff(r) as u32 + j.abs_diff(c) as u32;
                        if cand + min_save <= base {
                            ans += 1;
                        } else {
                            let to_start = from_start[r * m + c];
                            let to_end = from_end[i * m + j];
                            let cand =
                                to_start + to_end + i.abs_diff(r) as u32 + j.abs_diff(c) as u32;
                            if cand + min_save <= base {
                                ans += 1;
                            }
                        }
                    }
                }
            }
        }
        out.print_line(ans / 2);
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
//END MAIN
