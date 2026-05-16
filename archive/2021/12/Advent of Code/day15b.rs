use algo_lib::collections::iter_ext::IterExt;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::{Input, Readable};
use algo_lib::misc::dirs::D4;
use algo_lib::string::string::Str;

pub trait CommaList {
    fn read_list<T: Readable>(&mut self) -> Vec<T>;
}

impl CommaList for Input<'_> {
    fn read_list<T: Readable>(&mut self) -> Vec<T> {
        let mut s: String = self.read();
        s = s.replace(",", " ");
        let mut b = s.as_bytes();
        let input = Input::new(&mut b);
        input.into_iter().collect_vec()
    }
}

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut map = Vec::new();
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        map.push(inp.read::<Str>());
    }

    let n = map.len();
    let m = map[0].len();
    let mut graph = Graph::new(n * m * 25);
    for i in 0..(n * 5) {
        for j in 0..(m * 5) {
            for (r, c) in D4::iter(i, j, n * 5, m * 5) {
                let mut w = (map[r % n][c % m] - b'0') as u32;
                w += (r / n + c / m) as u32;
                while w > 9 {
                    w -= 9;
                }
                graph.add_edge(i * 5 * m + j, WeightedEdge::new(r * 5 * m + c, w));
            }
        }
    }

    println!("{}", graph.distance(0, 25 * n * m - 1).unwrap().0);
}
