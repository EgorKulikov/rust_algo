//{"name":"C. K-th LNCA","group":"Codeforces - Aleppo Collegiate Programming Contest 2023 V.2","url":"https://codeforces.com/gym/104544/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n7 1\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n2 4 4 5 6 7\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKThLNCA"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let lca = graph.lca();

    for _ in 0..q {
        let k = input.read_size();
        let m = input.read_size();
        let mut v = input.read_size_vec(m).dec();
        v.sort_by_key(|&v| lca.position(v));

        let mut ans = lca.lca(v[0], v[k - 1]);
        let mut qty = 1;
        for i in k..m {
            let cand = lca.lca(v[i - (k - 1)], v[i]);
            if lca.level(ans) < lca.level(cand) {
                qty = 1;
                ans = cand;
            }
            if lca.level(ans) == lca.level(cand) && ans != cand {
                ans = cand;
                qty += 1;
            }
        }
        out_line!(qty);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
