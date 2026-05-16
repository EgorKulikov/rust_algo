//{"name":"day17","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day17"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let lines = input.read_lines();

    let n = lines.len();
    let m = lines[0].len();
    let solve = |min_moves: usize, max_moves: usize| -> i64 {
        let encode = |i: usize, j: usize, d: usize, r: usize| {
            i * m * 4 * (max_moves + 1) + j * 4 * (max_moves + 1) + d * (max_moves + 1) + r
        };
        let mut graph = Graph::new(n * m * 4 * (max_moves + 1));
        for i in 0..n {
            for j in 0..m {
                for d in 0..4 {
                    for rem in 0..=max_moves {
                        if rem != 0 {
                            let (r, c) = D4::go(i, j, d, n, m);
                            if (r, c) != (i, j) {
                                graph.add_edge(WeightedEdge::new(
                                    encode(i, j, d, rem),
                                    encode(r, c, d, rem - 1),
                                    (lines[r][c] - b'0') as i64,
                                ));
                            }
                        }
                        if rem <= max_moves - min_moves {
                            for nd in [(d + 1) % 4, (d + 3) % 4] {
                                graph.add_edge(WeightedEdge::new(
                                    encode(i, j, d, rem),
                                    encode(i, j, nd, max_moves),
                                    0,
                                ));
                            }
                        }
                    }
                }
            }
        }
        let mut ans = None;
        for d in 0..2 {
            let dist = graph.distances_from(encode(0, 0, d, max_moves));
            for nd in 0..4 {
                for r in 0..=max_moves - min_moves {
                    if let Some((res, ..)) = dist[encode(n - 1, m - 1, nd, r)] {
                        ans.minim(res);
                    }
                }
            }
        }
        ans.unwrap()
    };

    {
        // part 1
        out.print_line(solve(1, 3));
    }

    {
        // part 2
        out.print_line(solve(4, 10));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
