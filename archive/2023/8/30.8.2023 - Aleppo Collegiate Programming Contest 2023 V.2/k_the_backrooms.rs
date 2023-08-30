//{"name":"K. The Backrooms","group":"Codeforces - Aleppo Collegiate Programming Contest 2023 V.2","url":"https://codeforces.com/gym/104544/problem/K","interactive":false,"timeLimit":3000,"tests":[{"input":"3 3\n1/2\n1 2\n2 3\n1 3\n","output":"571428578\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KTheBackrooms"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gauss::gauss;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    type Mod = ModInt7;

    let n = input.read_size();
    let m = input.read_size();
    let p = input
        .read_vec::<String>(n - 2)
        .into_iter()
        .map(|s| {
            let s = s.split("/").collect_vec();
            let p = s[0].parse::<i32>().unwrap();
            let q = s[1].parse::<i32>().unwrap();
            Mod::new(p) / Mod::new(q)
        })
        .collect_vec();
    let edges = input.read_size_pair_vec(m).dec();
    let graph = Graph::from_biedges(n, &edges);

    let mut ans = Arr2d::new(1 << (n - 2), n, Mod::zero());
    let mut g = Arr2d::new(n, n + 1, Mod::zero());
    for i in usize::iter_all(n - 2).rev() {
        g.fill(Mod::zero());
        for j in 0..n - 1 {
            g[(j, j)] = Mod::one();
            let mut mult = Mod::one();
            if j != 0 && !i.is_set(j - 1) {
                g[(j, n)] = (Mod::one() - p[j - 1]) * (Mod::new(2) + ans[(i.with_bit(j - 1), j)]);
                mult = p[j - 1];
            }
            g[(j, n)] += mult;
            let c = -mult / Mod::from_index(graph[j].len());
            for e in &graph[j] {
                g[(j, e.to())] = c;
            }
        }
        g[(n - 1, n - 1)] = Mod::one();
        gauss(&mut g);
        for j in 0..n {
            ans[(i, j)] = g[(j, n)];
        }
    }
    out_line!(ans[(0, 0)]);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
