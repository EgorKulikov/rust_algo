//{"name":"day23","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day23"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::memo::memoization::Memoization2;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let lines = input.read_lines();
    let n = lines.len();
    let m = lines[0].len();

    let mut crit = vec![(0, 1), (n - 1, m - 2)];
    for i in 0..n {
        for j in 0..m {
            if lines[i][j] == b'#' {
                continue;
            }
            let mut near = 0;
            for (r, c) in D4::iter(i, j, n, m) {
                if lines[r][c] != b'#' {
                    near += 1;
                }
            }
            if near > 2 {
                crit.push((i, j));
            }
        }
    }

    let solve = |any_direction: bool| -> i64 {
        let mut graph = Graph::new(crit.len());
        for i in crit.indices() {
            for (mut r, mut c) in D4::iter(crit[i].0, crit[i].1, n, m) {
                if lines[r][c] == b'#' {
                    continue;
                }
                let mut len = 1;
                let (mut lr, mut lc) = crit[i];
                'outer: loop {
                    if let Some(pos) = crit.iter().position(|&p| p == (r, c)) {
                        graph.add_edge(WeightedEdge::new(i, pos, len));
                        break;
                    }
                    len += 1;
                    let has_to = match lines[r][c] {
                        b'>' => Some((r, c + 1)),
                        b'<' => Some((r, c - 1)),
                        b'^' => Some((r - 1, c)),
                        b'v' => Some((r + 1, c)),
                        _ => None,
                    }
                    .filter(|_| !any_direction);
                    for (nr, nc) in D4::iter(r, c, n, m) {
                        if lines[nr][nc] != b'#' && (nr, nc) != (lr, lc) {
                            if let Some((tr, tc)) = has_to {
                                if (nr, nc) != (tr, tc) {
                                    break 'outer;
                                }
                            }
                            (lr, lc) = (r, c);
                            (r, c) = (nr, nc);
                            break;
                        }
                    }
                }
            }
        }

        let mut mem = Memoization2::new(|mem, cur: usize, mask: i64| -> i64 {
            if cur == 1 {
                return 0;
            }
            let mut res = i64::MIN / 2;
            for e in &graph[cur] {
                if !mask.is_set(e.to()) {
                    res.maxim(mem.call(e.to(), mask.with_bit(e.to())) + e.weight());
                }
            }
            res
        });
        mem.call(0, 1)
    };

    {
        // part 1
        out.print_line(solve(false));
    }

    {
        // part 2
        out.print_line(solve(true));
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
