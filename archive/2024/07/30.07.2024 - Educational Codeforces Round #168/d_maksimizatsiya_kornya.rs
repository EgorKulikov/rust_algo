//{"name":"D. Максимизация корня","group":"Codeforces - Educational Codeforces Round 168 (Rated for Div. 2)","url":"https://codeforces.com/contest/1997/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n4\n0 1 0 2\n1 1 3\n2\n3 0\n1\n5\n2 5 3 9 6\n3 1 5 2\n","output":"1\n3\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaksimizatsiyaKornya"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let p = input.read_size_vec(n - 1).dec();

    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        graph.add_edge(BiEdge::new(p[i], i + 1));
    }
    let mut left = 0;
    let mut right = *a[1..].iter().max().unwrap();
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut ok = true;
        for e in &graph[0] {
            let mut dfs =
                RecursiveFunction3::new(|f, vert: usize, prev: usize, mut need: i64| -> bool {
                    if a[vert] < need {
                        if graph[vert].len() == 1 {
                            return false;
                        } else {
                            need += need - a[vert];
                            if need >= i64::MAX / 2 {
                                return false;
                            }
                        }
                    }
                    for e in &graph[vert] {
                        if e.to() == prev {
                            continue;
                        }
                        if !f.call(e.to(), vert, need) {
                            return false;
                        }
                    }
                    true
                });
            if !dfs.call(e.to(), 0, mid) {
                ok = false;
                break;
            }
        }
        if ok {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    out.print_line(a[0] + left);
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
}
//END MAIN
