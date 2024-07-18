//{"name":"ucup21g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup21g"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::string::str::StrReader;
use algo_lib::when;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let w = input.read_str();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    when! {
        w.len() == 1 => {
            out.print_line(n);
        },
        w.len() == 2 && w[0] == w[1] => {
            out.print_line(2 * n - 2);
        },
        w.len() == 2 => {
            out.print_line(n - 1);
        },
        w[0] == w[1] && w[0] == w[2] => {
            if n == 1 {
                out.print_line(0);
                return;
            }
            let mut ans = 0;
            for i in 0..n {
                ans += graph[i].len() * (graph[i].len() - 1);
            }
            out.print_line(ans);
        },
        else => {
            let mut par = n;
            let mut mem = Memoization3d::new(n, 2, 2, |mem, vert, is_mid, par_mid| -> i64 {
                // eprintln!("{} {}", vert, par);
                let mut calls = Vec::new();
                for edge in &graph[vert] {
                    let next = edge.to();
                    if next == par {
                        continue;
                    }
                    let was_par = par;
                    par = vert;
                    calls.push((mem.call(next, 0, is_mid), mem.call(next, 1, is_mid)));
                    par = was_par;
                }
                if is_mid == 0 {
                    let mut ans = 0;
                    for (a, b) in calls {
                        ans += a.max(b);
                    }
                    // eprintln!("{} {} {} {}", vert, is_mid, par_mid, ans);
                    ans
                } else {
                    calls.sort_by_key(|&(a, b)| b - a);
                    let mut sum = 0;
                    for &(_, b) in &calls {
                        sum += b;
                    }
                    let calculate = |edge: usize, mid: usize| -> i64 {
                        (if w[0] == w[2] {
                            if edge == 0 {
                                0
                            } else {
                                edge * (edge - 1)
                            }
                        } else if w[0] == w[1] || w[1] == w[2] {
                            edge * mid
                        } else {
                            (edge / 2) * ((edge + 1) / 2)
                        }) as i64
                    };
                    let add_edge = 1 - par_mid;
                    let add_mid = if vert == 0 { 0 } else { par_mid };
                    let mut ans = 0;
                    for i in calls.indices() {
                        ans.maxim(sum + calculate(add_edge + i, calls.len() - i + add_mid));
                        let (a, b) = calls[i];
                        sum -= b;
                        sum += a;
                    }
                    ans.maxim(sum + calculate(add_edge + calls.len(), add_mid));
                    // eprintln!("{} {} {} {}", vert, is_mid, par_mid, ans);
                    ans
                }
            });
            out.print_line(mem.call(0, 0, 1).max(mem.call(0, 1, 1)));
        },
    }
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
    tester::stress_test();
}
//END MAIN
