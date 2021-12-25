use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::{Input, Readable};

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

    let top = inp.read_table::<char>(3, 13);
    let bottom = inp.read_table::<char>(2, 9);

    let tp = |j| {
        if j < 4 {
            (top[(2, 3 + 2 * j)] as usize) - ('A' as usize)
        } else {
            (bottom[(0, 1 + 2 * (j - 4))] as usize) - ('A' as usize)
        }
    };

    let mut graph = Graph::new(9usize.pow(8));
    for i in 0..graph.vertex_count() {
        if i % 1000000 == 0 {
            println!("{}", i);
        }
        let mut pos = Vec::with_capacity(8);
        let mut j = i;
        for _ in 0..8 {
            pos.push(j % 9);
            j /= 9;
        }
        for j in 0usize..8 {
            let t = tp(j);
            if j >= 4 && pos[j - 4] == 0 || pos[j] == 8 {
                continue;
            }
            if pos[j] == 0 {
                let after = 2 + j % 4;
                for k in 0usize..7 {
                    let (left, right) = if k < after {
                        (k, after)
                    } else {
                        (after, k + 1)
                    };
                    let mut found = false;
                    for l in pos.iter().cloned() {
                        if l >= left + 1 && l < right + 1 {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        let mut steps = 2 + j / 4;
                        for l in left..(right - 1) {
                            if l == 0 || l == 5 {
                                steps += 1;
                            } else {
                                steps += 2;
                            }
                        }
                        graph.add_edge(
                            i,
                            WeightedEdge::new(
                                i + (k + 1) * 9usize.pow(j as u32),
                                steps * 10usize.pow(t as u32),
                            ),
                        );
                    }
                }
                if j % 4 == t && (j >= 4 || pos[j + 4] == 8) {
                    graph.add_edge(i, WeightedEdge::new(i + 8 * 9usize.pow(j as u32), 0));
                }
            } else if pos[j] < 8 {
                if pos[t] == 0 || pos[t + 4] == 0 {
                    continue;
                }
                let after = 2 + t;
                let (left, right) = if pos[j] - 1 < after {
                    (pos[j] - 1, after)
                } else {
                    (after, pos[j])
                };
                let mut found = false;
                for l in pos.iter().cloned() {
                    if l >= left + 1 && l < right + 1 && l != pos[j] {
                        found = true;
                        break;
                    }
                }
                if !found {
                    let mut steps = 3;
                    for l in left..(right - 1) {
                        if l == 0 || l == 5 {
                            steps += 1;
                        } else {
                            steps += 2;
                        }
                    }
                    for (k, l) in pos.iter().cloned().enumerate() {
                        if tp(k) == t && l == 8 {
                            steps -= 1;
                        }
                    }
                    graph.add_edge(
                        i,
                        WeightedEdge::new(
                            i + (8 - pos[j]) * 9usize.pow(j as u32),
                            steps * 10usize.pow(t as u32),
                        ),
                    );
                }
            }
        }
    }

    let mut res = vec![usize::MAX; graph.vertex_count()];
    res[0] = 0;
    for i in 0..res.len() {
        if res[i] == usize::MAX {
            continue;
        }
        for e in graph[i].iter() {
            let cand = res[i] + e.weight();
            res[e.to()].minim(cand);
        }
    }
    println!("{}", res.last().unwrap());
}
