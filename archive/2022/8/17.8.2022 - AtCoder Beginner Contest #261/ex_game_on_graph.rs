//{"name":"Ex - Game on Graph","group":"AtCoder - AtCoder Beginner Contest 261","url":"https://atcoder.jp/contests/abc261/tasks/abc261_h","interactive":false,"timeLimit":2000,"tests":[{"input":"7 6 1\n1 2 1\n1 3 10\n2 4 100\n2 5 102\n3 6 20\n3 7 30\n","output":"40\n"},{"input":"3 6 3\n1 2 1\n2 1 2\n2 3 3\n3 2 4\n3 1 5\n1 3 6\n","output":"INFINITY\n"},{"input":"4 4 1\n1 2 1\n2 3 1\n3 1 1\n2 4 1\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ExGameOnGraph"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BTreeSet;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let v = input.read_usize() - 1;
    let edges = input.read_vec::<(usize, usize, i64)>(m);

    let mut graph = Graph::new(n);
    let mut rem = vec![0; n];
    let mut aoki = vec![0; n];
    let mut tako = vec![0; n];
    const INF: i64 = std::i64::MAX;
    for (a, b, c) in edges {
        graph.add_edge(b - 1, WeightedEdge::new(a - 1, c));
        rem[a - 1] += 1;
        tako[a - 1] = INF;
    }
    let mut queue = BTreeSet::new();
    #[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
    enum Player {
        Aoki,
        Tako,
    }
    for i in 0..n {
        if tako[i] == 0 {
            queue.insert((aoki[i], Player::Aoki, i));
            queue.insert((tako[i], Player::Tako, i));
        }
    }
    let mut done = BitSet::new(n);
    while let Some(&(v, player, i)) = queue.iter().next() {
        queue.remove(&(v, player, i));
        match player {
            Player::Aoki => {
                for e in &graph[i] {
                    if !done[e.to()] {
                        queue.remove(&(tako[e.to()], Player::Tako, e.to()));
                        tako[e.to()].minim(aoki[i] + e.weight());
                        queue.insert((tako[e.to()], Player::Tako, e.to()));
                    }
                }
            }
            Player::Tako => {
                done.set(i, true);
                for e in &graph[i] {
                    rem[e.to()] -= 1;
                    aoki[e.to()].maxim(tako[i] + e.weight());
                    if rem[e.to()] == 0 {
                        queue.insert((aoki[e.to()], Player::Aoki, e.to()));
                    }
                }
            }
        }
    }
    if tako[v] == INF {
        out_line!("INFINITY");
    } else {
        out_line!(tako[v]);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
