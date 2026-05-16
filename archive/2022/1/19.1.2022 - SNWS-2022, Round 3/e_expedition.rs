//{"name":"E. Expedition","group":"Yandex - SNWS-2022, Round 3","url":"https://contest.yandex.ru/snws2022/contest/23959/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 1 1\n1 2\n2 3\n1 3\n1 4\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EExpedition"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let x = input.read_usize();
    let y = input.read_usize();
    let edges = input.read_vec::<(usize, usize)>(m).dec_by_one();

    let mut graph = Graph::new(n);
    let set = edges.into_iter().collect::<HashSet<_>>();
    for u in 0..n {
        for v in 0..u {
            if !set.contains(&(u, v)) && !set.contains(&(v, u)) {
                graph.add_edge(u, BiEdge::new(v));
            }
        }
    }
    let mut pairs = Vec::new();
    let mut free = 0;
    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] == 0 {
            let mut dfs = RecursiveFunction2::new(|f, vert, col: i32| {
                if color[vert] != 0 {
                    return if color[vert] == col {
                        Some((0, 0))
                    } else {
                        None
                    };
                }
                color[vert] = col;
                let mut white = 1;
                let mut black = 0;
                for e in &graph[vert] {
                    match f.call(e.to(), -col) {
                        Some((b, w)) => {
                            white += w;
                            black += b;
                        }
                        None => {
                            return None;
                        }
                    }
                }
                Some((white, black))
            });
            match dfs.call(i, 1) {
                None => {
                    out_line!(0);
                    return;
                }
                Some(p) => {
                    if p == (1, 0) {
                        free += 1;
                    } else {
                        pairs.push(p);
                    }
                }
            }
        }
    }
    let rem = n - free;
    type Mod = ModIntF;
    let mut ans = vec![Mod::zero(); rem + 1];
    ans[0] = Mod::one();
    for (w, b) in pairs {
        assert!(w > 0);
        assert!(b > 0);
        for i in (0..=rem).rev() {
            let cur = ans[i];
            if i + w <= rem {
                ans[i + w] += cur;
            }
            if i + b <= rem {
                ans[i + b] += cur;
            }
            ans[i] = Mod::zero();
        }
    }
    let mut res = Mod::zero();
    let c: Combinations<Mod> = Combinations::new(free + 1);
    for (i, a) in ans.into_iter().enumerate() {
        for j in 0..=free {
            if i + j < x {
                continue;
            }
            for k in 0..=j {
                if rem - i + free - k < y {
                    break;
                }
                res += a * c.c(free, j) * c.c(j, k);
            }
        }
    }
    out_line!(res);
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
