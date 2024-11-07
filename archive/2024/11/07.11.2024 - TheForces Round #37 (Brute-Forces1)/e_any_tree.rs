//{"name":"E. Any Tree ?","group":"Codeforces - TheForces Round #37 (Brute-Forces1)","url":"https://codeforces.com/gym/105491/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n1 1 1 1\n1 2 1 1\n1 1 3 3\n1 1 3 4\n5\n1 1 1 1 1\n1 2 1 1 1\n1 1 3 3 3\n1 1 3 4 3\n1 1 3 3 5\n5\n1 1 1 1 2\n3 4 1 1 3\n3 4 1 1 5\n2 3 1 2 3\n1 2 3 4 5\n6\n1 1 1 1 1 1\n1 2 2 1 1 2\n1 2 3 2 3 1\n1 1 2 4 3 2\n1 1 3 3 5 2\n1 2 1 2 2 6\n7\n1 1 1 1 1 1 1\n1 2 1 2 1 1 1\n1 1 3 1 3 3 3\n1 2 1 4 1 1 1\n1 1 3 1 5 5 3\n1 1 3 1 5 6 3\n1 1 3 1 3 3 7\n","output":"3 4\n2 1\n1 3\n3 5\n1 2\n4 3\n3 1\n-1\n-1\n2 4\n3 1\n3 6\n4 1\n7 3\n5 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAnyTree"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input
        .read_size_table(n, n)
        .do_with(|a| a.iter_mut().for_each(|x| *x -= 1));

    let mut ans = Vec::with_capacity(n - 1);
    for i in (1..n).rev() {
        for j in (0..i).rev() {
            if a[(i, j)] == j {
                ans.push((i, j));
                break;
            }
        }
    }
    if ans.len() != n - 1 {
        out.print_line(-1);
        return;
    }
    let mut good = true;
    let graph = Graph::from_biedges(n, &ans);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Vec<usize> {
        if a[(vert, vert)] != vert {
            good = false;
            return vec![];
        }
        let mut v = vec![vert];
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            if !good {
                return vec![];
            }
            for &i in &call {
                for &j in &v {
                    if a[(i, j)] != vert || a[(j, i)] != vert {
                        good = false;
                        return vec![];
                    }
                }
            }
            v.extend(call);
        }
        v
    });
    dfs.call(0, n);
    if good {
        out.print_per_line(&ans.inc());
    } else {
        out.print_line(-1);
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
