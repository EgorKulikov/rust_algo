//{"name":"E. Камень, ножницы, бумага","group":"Yandex - Yandex Cup 2022: Алгоритм, спринт (квалификация)","url":"https://contest.yandex.ru/yacup/contest/42199/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 2 D\n1 3 W\n3 2 L\n","output":"9\n3\n3\n"},{"input":"2 2\n1 2 W\n2 1 W\n","output":"3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKamenNozhnitsiBumaga"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let edges = input.read_vec::<(usize, usize, char)>(q);

    let mut left = 0;
    let mut right = q;
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut graph = Graph::new(n);
        for &(mut u, mut v, c) in edges.iter().take(mid) {
            u -= 1;
            v -= 1;
            graph.add_edge(
                u,
                WeightedEdge::new(
                    v,
                    match c {
                        'D' => 0,
                        'W' => 1,
                        'L' => 2,
                        _ => unreachable!(),
                    },
                ),
            );
            graph.add_edge(
                v,
                WeightedEdge::new(
                    u,
                    match c {
                        'D' => 0,
                        'W' => 2,
                        'L' => 1,
                        _ => unreachable!(),
                    },
                ),
            );
        }
        let mut color = vec![4; n];
        let mut good = true;
        for i in 0..n {
            if color[i] != 4 {
                continue;
            }
            let mut rec = RecursiveFunction2::new(|f, vert, res| {
                if color[vert] != 4 {
                    if color[vert] != res {
                        good = false;
                    }
                    return;
                }
                color[vert] = res;
                for e in &graph[vert] {
                    f.call(e.to(), (res + e.weight()) % 3);
                }
            });
            rec.call(i, 0);
        }
        if good {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    let mut dsu = DSU::new(n);
    type Mod = ModInt7;
    for &(u, v, _) in edges.iter().take(left) {
        let u = u - 1;
        let v = v - 1;
        dsu.join(u, v);
        out_line!(Mod::new(3).power(dsu.count()));
    }
    for _ in left..q {
        out_line!(0);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
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
