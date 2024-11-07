//{"name":"F. Permutation via Tree","group":"Codeforces - TheForces Round #37 (Brute-Forces1)","url":"https://codeforces.com/gym/105491/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n3\n1 3\n1 2\n5\n4 5\n1 4\n3 1\n2 4\n","output":"2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPermutationViaTree"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    type Mod = ModInt7;
    let graph = Graph::from_biedges(n, &edges);
    let c = Combinations::<Mod>::new(n + 1);
    let mut good = true;
    let mut dfs =
        RecursiveFunction3::new(|f, vert: usize, prev: usize, parts: usize| -> Vec<Mod> {
            let mut res = vec![Mod::zero(); parts + 2];
            let children = graph[vert].len() - if vert == 0 { 0 } else { 1 };
            if parts + 1 < children {
                good = false;
                return vec![];
            }
            res[0] = Mod::one();
            if children == 0 {
                res.pop();
                res.pop();
                return res;
            }
            let mut shift = 0usize;
            let mut nres = vec![Mod::zero(); parts + 2];
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let ways = f.call(e.to(), vert, parts + 1 - (children - 1));
                if !good {
                    return vec![];
                }
                nres[shift.saturating_sub(1)..].fill(Mod::zero());
                for i in shift..res.len() {
                    for j in ways.indices() {
                        if i + j + 1 < nres.len() {
                            nres[i + j + 1] += res[i] * ways[j];
                        }
                    }
                }
                swap(&mut res, &mut nres);
                shift += 1;
            }
            nres.pop();
            nres.pop();
            for i in nres.indices() {
                nres[i] = res[i]
                    + res[i + 1] * Mod::from_index(2 * (i + 1))
                    + res[i + 2] * c.c(i + 2, 2) * Mod::new(2);
            }
            nres
        });
    let ans = dfs.call(0, 0, 1);
    // assert_eq!(ans.len(), 1);
    if !good {
        out.print_line(0);
    } else {
        out.print_line(ans[0]);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
