//{"name":"D. Саша и прогулка по городу","group":"Codeforces - Codeforces Round 926 (Div. 2)","url":"https://codeforces.com/contest/1929/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 3\n3 2\n4\n3 4\n2 3\n3 1\n5\n1 2\n3 4\n5 1\n2 3\n4\n1 2\n2 3\n3 4\n","output":"7\n12\n16\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSashaIProgulkaPoGorodu"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    type Mod = ModIntF;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Mod, Mod) {
        let mut sum_ones = Mod::zero();
        let mut sum_twos = Mod::zero();
        let mut prod_ones = Mod::one();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (one, two) = f.call(e.to(), vert);
            sum_ones += one;
            sum_twos += two;
            prod_ones *= one + Mod::one();
        }
        prod_ones -= Mod::one();
        let one = prod_ones + Mod::one();
        let two = sum_twos + sum_ones;
        (one, two)
    });
    let (one, two) = dfs.call(0, n);
    out.print_line(one + two + Mod::one());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
