//{"name":"haunted_house_part_2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::lcm;
use algo_lib::output;
use algo_lib::string::str::StrReader;
use std::iter::once;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    if test_case != 1 {
        input.read_str();
    }
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let b = input.read_size();
    let s = input.read_size();
    let g = input.read_char_table(n, m);
    let rcd = input.read_vec::<(usize, usize, u8)>(k);

    let l = lcm(n, m);
    let mut forbidden = FxHashSet::default();
    let mut forbidden_pair = FxHashSet::default();
    for i in 0..k {
        let (mut r, mut c, d) = rcd[i];
        for j in 0..l {
            forbidden.insert((j, (r, c)));
            let (nr, nc) = match d {
                b'U' => ((r + n - 1) % n, c),
                b'D' => ((r + 1) % n, c),
                b'L' => (r, (c + m - 1) % m),
                b'R' => (r, (c + 1) % m),
                _ => unreachable!(),
            };
            forbidden_pair.insert((j, (nr, nc), (r, c)));
            r = nr;
            c = nc;
        }
    }
    let mut graph = Graph::new(l * n * m * (b + 1) * s);
    let id = |t, r, c, bb, ss| (t * n * m + r * m + c) * (b + 1) * s + bb * s + ss;
    for t in 0..l {
        for (r, c) in g.indices() {
            if g[(r, c)] == b'X' || forbidden.contains(&(t, (r, c))) {
                continue;
            }
            for bb in 0..=b {
                for ss in 0..s {
                    let from = id(t, r, c, bb, ss);
                    for (nr, nc) in D4::iter(r, c, n, m).chain(once((r, c))) {
                        if g[(nr, nc)] == b'X' {
                            continue;
                        }
                        let no_freeze = !forbidden.contains(&((t + 1) % l, (nr, nc)))
                            && !forbidden_pair.contains(&(t, (r, c), (nr, nc)));
                        let freeze = !forbidden.contains(&(t % l, (nr, nc)));
                        if freeze {
                            if ss > 0 {
                                let to = id(t, nr, nc, bb, ss - 1);
                                graph.add_edge(Edge::new(from, to));
                            } else if bb > 0 {
                                let to = id(t, nr, nc, bb - 1, s - 1);
                                graph.add_edge(Edge::new(from, to));
                            }
                        }
                        if no_freeze && ss == 0 {
                            let to = id((t + 1) % l, nr, nc, bb, 0);
                            graph.add_edge(Edge::new(from, to));
                        }
                    }
                }
            }
        }
    }
    let mut start = 0;
    let mut end = 0;
    for (r, c) in g.indices() {
        if g[(r, c)] == b'S' {
            start = r * m + c;
        }
        if g[(r, c)] == b'E' {
            end = r * m + c;
        }
    }
    let mut ans = u32::MAX;
    let d = graph.edge_distances(id(0, start / m, start % m, b, 0));
    for t in 0..l {
        for bb in 0..=b {
            for ss in 0..s {
                let dist = d[id(t, end / m, end % m, bb, ss)];
                ans.minim(dist);
            }
        }
    }
    if ans == u32::MAX {
        output!(out, "Case #{}: -1", test_case);
    } else {
        output!(out, "Case #{}: {}", test_case, ans);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                eprintln!("Processing test case {}", i);
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
