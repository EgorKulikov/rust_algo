//{"name":"#5 - Slučajna Cesta","group":"DMOJ - COCI '23 Contest 3","url":"https://dmoj.ca/problem/coci23c3p5","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n1\n2 1\n","output":"500000006\n2\n"},{"input":"3\n1 1\n8 8 8\n","output":"14\n14\n14\n"},{"input":"11\n1 1 1 2 3 4 1 2 6 2\n1 1000 5 3 18 200 8 9 0 2 2\n","output":"968750272\n610352580\n450521029\n536458466\n199219275\n662760680\n190972315\n90277951\n824219264\n941840425\n532552597\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SluajnaCesta"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable, Callable2, RecursiveFunction, RecursiveFunction2,
};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    type Mod = ModInt7;

    let n = input.read_size();
    let p = input.read_size_vec(n - 1).dec();
    let v = input.read_vec::<Mod>(n);

    let mut graph = Graph::new(n);
    for (i, p) in p.into_iter().enumerate() {
        graph.add_edge(Edge::new(p, i + 1));
    }

    let mut down = vec![Mod::zero(); n];
    let mut sum_down = vec![Mod::zero(); n];
    let mut dfs = RecursiveFunction::new(|f, vert: usize| {
        for e in &graph[vert] {
            sum_down[vert] += f.call(e.to());
        }
        if graph[vert].len() != 0 {
            down[vert] = (Mod::one() - Mod::one() / Mod::new(2).power(graph[vert].len()))
                * sum_down[vert]
                / Mod::from_index(graph[vert].len())
                + v[vert];
        } else {
            down[vert] = v[vert];
        }
        down[vert]
    });
    dfs.call(0);
    let mut up = vec![Mod::zero(); n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, from_up: Mod| {
        up[vert] = from_up;
        let len = if vert == 0 {
            graph[vert].len() - 1
        } else {
            graph[vert].len()
        };
        let c = if len == 0 {
            Mod::zero()
        } else {
            (Mod::one() - Mod::one() / Mod::new(2).power(len)) / Mod::from_index(len)
        };
        for e in &graph[vert] {
            let cur_sum = sum_down[vert] + from_up - down[e.to()];
            f.call(e.to(), c * cur_sum + v[vert]);
        }
    });
    dfs.call(0, Mod::zero());
    let mut ans = vec![down[0]];
    for i in 1..n {
        let len = graph[i].len() + 1;
        let c = (Mod::one() - Mod::one() / Mod::new(2).power(len)) / Mod::from_index(len);
        let cur_sum = sum_down[i] + up[i];
        ans.push(c * cur_sum + v[i]);
    }
    out.print_per_line(&ans);
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
    //    tester::stress_test();
}
//END MAIN
