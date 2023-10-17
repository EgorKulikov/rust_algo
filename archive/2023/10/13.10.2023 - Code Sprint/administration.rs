//{"name":"Administration","group":"CodeChef - COSS2023","url":"https://www.codechef.com/COSS2023/problems/ADMN","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2\n4 1\n4 5\n1 2\n2 3\n","output":"1\n"},{"input":"10 2\n2 6\n6 7\n7 3\n3 10\n10 5\n5 8\n8 1\n1 4\n4 9\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Administration"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let d = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(
        |f, vert: usize, prev: usize| -> (Option<usize>, Option<usize>) {
            let mut uncovered = None;
            let mut governed = None;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let (call_uncovered, call_governed) = f.call(e.to(), vert);
                if let Some(call_uncovered) = call_uncovered {
                    uncovered.maxim(call_uncovered + 1);
                }
                if let Some(call_governed) = call_governed {
                    governed.minim(call_governed + 1);
                }
            }
            if governed.is_none() || governed.unwrap() > d {
                uncovered.maxim(0);
            }
            if uncovered == Some(d) {
                ans += 1;
                uncovered = None;
                governed = Some(0);
            } else if let (Some(u_uncovered), Some(u_governed)) = (uncovered, governed) {
                if u_uncovered + u_governed > d {
                    governed = None;
                } else {
                    uncovered = None;
                }
            }
            (uncovered, governed)
        },
    );
    let (uncovered, _) = dfs.call(0, 0);
    if uncovered.is_some() {
        ans += 1;
    }
    out.print_line(ans);
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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
