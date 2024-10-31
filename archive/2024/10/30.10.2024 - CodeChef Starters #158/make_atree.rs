//{"name":"Make a Tree","group":"CodeChef - START158A","url":"https://www.codechef.com/START158A/problems/MAKETREE","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n1 2\n3\n3 2\n1 2\n","output":"1\n1 1\n2\n1 1 2\n3 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MakeATree"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let k = (0..n).map(|i| graph[i].len()).max().unwrap();
    let mut edges = vec![Vec::new(); k];
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, forbidden: usize| {
        let mut next = if forbidden == 0 { 1 } else { 0 };
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            edges[next].push((vert, e.to()));
            f.call(e.to(), vert, next);
            next += 1;
            if next == forbidden {
                next += 1;
            }
        }
    });
    dfs.call(0, 0, k);
    let mut ans = Arr2d::new(k, n, 0);
    for i in 0..k {
        let mut next = 1;
        for &(a, b) in &edges[i] {
            ans[(i, a)] = next;
            ans[(i, b)] = next;
            next += 1;
        }
        for j in 0..n {
            if ans[(i, j)] == 0 {
                ans[(i, j)] = next;
                next += 1;
            }
        }
    }
    out.print_line(k);
    out.print_line(ans);
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
