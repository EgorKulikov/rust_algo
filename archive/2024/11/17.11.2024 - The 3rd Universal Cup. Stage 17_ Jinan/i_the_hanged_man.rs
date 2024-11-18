//{"name":"I. The Hanged Man","group":"Universal Cup - The 3rd Universal Cup. Stage 17: Jinan","url":"https://contest.ucup.ac/contest/1843/problem/9556","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 2\n2 3\n2 4\n7\n1 2\n1 3\n1 4\n4 5\n4 6\n4 7\n6\n1 2\n2 3\n2 4\n1 5\n5 6\n","output":"-1\n3\n3 2\n7 6\n5 1\n2\n4 3\n6 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ITheHangedMan"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    if n == 2 {
        out.print_line(-1);
        return;
    }
    let graph = Graph::from_biedges(n, &edges);
    for i in 0..n {
        if graph[i].len() % 2 == 1 {
            continue;
        }
        let mut ans = Vec::new();
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Option<usize> {
            if graph[vert].len() == 1 {
                return Some(vert);
            }
            let mut calls_down = Vec::new();
            let mut call_further = Vec::new();
            let mut free = None;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let call = f.call(e.to(), vert);
                if let Some(call) = call {
                    if call == e.to() && vert == i {
                        calls_down.push(e.to());
                    } else {
                        call_further.push(call);
                    }
                } else {
                    free = Some(e.to());
                }
            }
            if vert != i {
                for i in (0..call_further.len() / 2 * 2).step_by(2) {
                    ans.push((call_further[i] + 1, call_further[i + 1] + 1));
                }
                call_further
                    .get(call_further.len() / 2 * 2)
                    .copied()
                    .or(Some(vert))
            } else {
                assert!(!call_further.is_empty() || free.is_some() || calls_down.len() % 2 == 0);
                for i in (0..calls_down.len() / 2 * 2).step_by(2) {
                    ans.push((calls_down[i] + 1, calls_down[i + 1] + 1));
                }
                let mut call_down = calls_down.get(calls_down.len() / 2 * 2).copied();
                if let Some(cd) = call_down {
                    if let Some(x) = call_further.pop() {
                        ans.push((cd + 1, x + 1));
                        call_down = None;
                    }
                }
                for i in (0..call_further.len() / 2 * 2).step_by(2) {
                    ans.push((call_further[i] + 1, call_further[i + 1] + 1));
                }
                let call_further = call_further.get(call_further.len() / 2 * 2).copied();
                if let Some(call_further) = call_further {
                    if let Some(call_down) = call_down {
                        ans.push((call_further + 1, call_down + 1));
                    } else {
                        ans.push((call_further + 1, i + 1));
                    }
                } else if let Some(call_down) = call_down {
                    ans.push((free.unwrap() + 1, call_down + 1));
                }
                None
            }
        });
        if dfs.call(i, n).is_some() {
            out.print_line(-1);
        } else {
            out.print_line(ans.len());
            out.print_per_line(&ans);
        }
        return;
    }
    out.print_line(-1);
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
