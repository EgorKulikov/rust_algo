//{"name":"WOSS Dual Olympiad 2021 S3: Late for Class!","group":"DMOJ","url":"https://dmoj.ca/problem/wossoly2021s3","interactive":false,"timeLimit":2000,"tests":[{"input":"7 7\n.....3.\n.......\n.0.....\n.......\n......1\n...2...\n.....4.\n","output":"24\n"},{"input":"10 10\n##########\n#0.....1.#\n#....###.#\n#..3.#2..#\n#....###.#\n#........#\n#..###...#\n#..#4....#\n#..##....#\n##########\n","output":"31\n"},{"input":"17 18\n...#X#.........#x4\n####.#.........###\n#k1..#............\n####.#............\n...#.#....j2......\n...#J#............\n...###............\n..................\n..................\n..................\n...........#......\n...........#..##..\n...........#..#K..\n...........#..##..\n....3......#.####.\n...............0#.\n.........########.\n","output":"66\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WOSSDualOlympiad2021S3LateForClass"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let r = input.read();
    let c = input.read();
    let grid = input.read_table::<char>(r, c);

    let mut path = [0usize; 5];
    let mut portals = [0usize; 26];
    for i in 0..r {
        for j in 0..c {
            let cur = grid[(i, j)];
            if cur.is_numeric() {
                path[(cur as usize) - ('0' as usize)] = i * c + j;
            }
            if cur.is_lowercase() {
                portals[(cur as usize) - ('a' as usize)] = i * c + j;
            }
        }
    }

    let mut graph = Graph::new(r * c);
    for i in 0..r {
        for j in 0..c {
            let cur = grid[(i, j)];
            if cur == '.' || cur.is_numeric() || cur.is_lowercase() {
                for (nr, nc) in D4::iter(i, j, r, c) {
                    let next = grid[(nr, nc)];
                    if next == '.' || next.is_numeric() {
                        graph.add_edge(i * c + j, Edge::new(nr * c + nc));
                    } else if next.is_uppercase() {
                        graph.add_edge(
                            i * c + j,
                            Edge::new(portals[(next as usize) - ('A' as usize)]),
                        );
                    }
                }
            }
        }
    }
    let mut ans = 0u32;
    for i in 0usize..4 {
        let dist = graph.edge_distances(path[i])[path[i + 1]];
        if dist == std::u32::MAX {
            eprintln!("{}", i);
            out_line!(-1);
            return;
        }
        ans += dist;
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
