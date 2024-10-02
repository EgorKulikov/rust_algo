//{"name":"F. Red Blue Tree","group":"Codeforces - TheForces Round #35 (LOL-Forces)","url":"https://codeforces.com/gym/105390/problem/F","interactive":false,"timeLimit":1500,"tests":[{"input":"6\n2 4 3 6 5 1\n011001\n5 4\n3 1\n6 5\n1 5\n2 3\n","output":"1\n"},{"input":"7\n9 10 11 5 2 3 6\n0010110\n3 1\n6 4\n1 2\n3 5\n4 7\n4 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRedBlueTree"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let s = input.read_str();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut dfs =
        RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Vec<i64>, Option<usize>) {
            let mut min_beauty = vec![if s[vert] == b'0' { a[vert] } else { -a[vert] }];
            let mut all_red = 0;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let (call_min_beauty, mut call_all_red) = f.call(e.to(), vert);
                let mut cut_cost = call_all_red;
                for i in call_min_beauty.indices() {
                    if call_min_beauty[i] < 0 {
                        call_all_red.minim(i + 1);
                        cut_cost.minim(i);
                        break;
                    }
                }
                let cut_cost = cut_cost.unwrap();
                let mut next_min_beauty =
                    vec![i64::MAX / 2; min_beauty.len() + call_min_beauty.len()];
                for i in min_beauty.indices() {
                    next_min_beauty[i + cut_cost + 1].minim(min_beauty[i]);
                    for j in call_min_beauty.indices() {
                        let k = i + j;
                        next_min_beauty[k].minim(min_beauty[i] + call_min_beauty[j]);
                    }
                }
                min_beauty = next_min_beauty;
                all_red += call_all_red.unwrap();
            }
            // eprintln!("{}: {:?} {}", vert, min_beauty, all_red);
            (
                min_beauty,
                if s[vert] == b'0' { Some(all_red) } else { None },
            )
        });
    let (min_beauty, all_red) = dfs.call(0, n);
    let mut ans = all_red;
    for i in 0..=n - 1 {
        if min_beauty[i] < 0 {
            ans.minim(i);
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
