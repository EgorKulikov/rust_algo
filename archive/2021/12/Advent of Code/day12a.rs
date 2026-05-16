use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::id::Id;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::{Input, Readable};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

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

    let mut id = Id::new();
    let mut edges = Vec::new();
    loop {
        inp.skip_whitespace();
        if inp.is_exhausted() {
            break;
        }
        let string = inp.read::<String>();
        let tokens = string.split("-").collect_vec();
        assert!(tokens.len() == 2);
        let left = tokens[0];
        let right = tokens[1];
        edges.push((id.get(left.to_string()), id.get(right.to_string())));
    }

    let n = id.len();
    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut small = vec![usize::MAX; n];
    let mut at = 0;
    for (i, v) in id.by_id().into_iter().enumerate() {
        if v.to_lowercase() == v {
            small[i] = at;
            at += 1;
        }
    }

    let start = id.get("start".to_string());
    let end = id.get("end".to_string());
    let mut ans = Arr2d::new(n, 1 << small.len(), -1i64);
    let mut rec = RecursiveFunction2::new(|f, vert, mut mask: usize| {
        if small[vert] < n {
            if mask.is_set(small[vert]) {
                return 0;
            } else {
                mask.set_bit(small[vert]);
            }
        }
        if ans[(vert, mask)] != -1 {
            ans[(vert, mask)]
        } else {
            if vert == end {
                ans[(vert, mask)] = 1;
                1
            } else {
                let mut res = 0;
                for e in graph[vert].iter() {
                    res += f.call(e.to(), mask);
                }
                ans[(vert, mask)] = res;
                res
            }
        }
    });
    println!("{}", rec.call(start, 0));
}
